use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::pool::db_manager::DbManager;
use crate::sql::executor::{Executor, RowType};
use deadpool::managed::Object;
use deadpool_sqlite::{Manager, Pool};
use rusqlite::types::ValueRef;
use rusqlite::{Row, ToSql};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task_local;

task_local! {
    pub static SQLITE_CONN_REGISTER : Arc<Mutex<Object<Manager>>>;
    pub static SQLITE_TX_REGISTER : Arc<Mutex<Option<rusqlite::Transaction<'static>>>>;
}
#[derive(Clone)]
pub struct SqliteSqlExecutor;
// 全局单例
pub const SQLITE_SQL_EXECUTOR: SqliteSqlExecutor = SqliteSqlExecutor;
// 提取公共函数避免重复
async fn query<T, R, F, Q>(
    conn: &deadpool_sync::SyncWrapper<rusqlite::Connection>,
    sql: String,
    params: Vec<ParamValue>,
    mapper: F,
    processor: Q,
) -> Result<R, DatabaseError>
where
    T: Send + 'static,
    R: Send + 'static,
    F: for<'a> Fn(&rusqlite::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
    Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
{
    conn.interact(move |conn| {
        let mut stmt = conn.prepare(sql.as_str())?;
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();

        let mut rows = stmt.query(&*param_refs)?;
        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(mapper(&row)?);
        }

        processor(results)
    }).await.map_err(|e| DatabaseError::CommonError(format!("Database interaction failed: {:?}", e)))?

}

async fn execute(
    conn: &deadpool_sync::SyncWrapper<rusqlite::Connection>,
    sql: String,
    params: Vec<ParamValue>,
) -> Result<u64, DatabaseError>
{   conn.interact(move |conn| {
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();
        let res = conn.execute(sql.as_str(), &*param_refs)?;
        Ok(res as u64)
    }).await.map_err(|e| DatabaseError::CommonError(format!("Database interaction failed: {:?}", e)))?

}

impl<'a> RowType for rusqlite::Row<'a> {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.get_ref(col_index)?;
        Ok(value_to_param_value(val)?)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.get_ref(col_name)?;
        Ok(value_to_param_value(val)?)
    }
}

// 查询基本实现

impl Executor for SqliteSqlExecutor {
    type Row<'a> = rusqlite::Row<'a>;
    async fn exec_basic(sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let conn_ref = SQLITE_CONN_REGISTER.try_get();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            let conn = conn_ref.lock().await;
            execute(conn.as_ref(), sql, params).await
        } else {
            let conn:Object<Manager> = get_conn().await;
            execute(conn.as_ref(), sql, params).await
        }
    }

    async fn query_basic<T, R, F, Q>(
        &self,
        sql: String,
        params: Vec<ParamValue>,
        mapper: F,
        processor: Q,
    ) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&rusqlite::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static{

        let conn_ref = SQLITE_CONN_REGISTER.try_get();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            let conn = conn_ref.lock().await;
            let conn = conn.as_ref();
            query(conn, sql, params, mapper, processor).await // 现在可以借用
        } else {
            let conn:Object<Manager> = get_conn().await;
            query(conn.as_ref(), sql, params, mapper, processor).await
        }

    }

    // fn row_to_e< E>(row: ) -> Result<E, DatabaseError> where E: Entity {
    //     let mut e = E::new();
    //     for col in E::column_names() {
    //         let val = row.get_ref(col)?;
    //         let param_value = value_to_param_value(val)?;
    //         e.set_value_by_column_name(col, param_value);
    //     }
    //     Ok(e)
    // }

    // fn col_to_v_by_name(row: &dyn RowType, col_name: &str) -> Result<ParamValue, DatabaseError> {
    //     let val = row.get_ref(col_name).map_err(|e| DatabaseError::CommonError(format!("Failed to get column {}: {:?}", col_name, e)))?;
    //     value_to_param_value(val)
    // }

    // fn col_to_v_by_index(row: &dyn RowType, col_index: usize) -> Result<ParamValue, DatabaseError> {
    //     let val = row.get_ref(col_index).map_err(|e| DatabaseError::CommonError(format!("Failed to get column at index {}: {:?}", col_index, e)))?;
    //     value_to_param_value(val)
    // }

    // async fn start_transaction(&self) -> Result<(), DatabaseError> {
    //     let conn_ref = SQLITE_CONN_REGISTER.try_get();
    //     if conn_ref.is_ok() {
    //         let conn_ref = conn_ref.unwrap().clone();
    //         let mut conn = conn_ref.lock().await;
    //         let conn = conn.as_ref();
    //
    //         // 开始事务
    //         let tx = conn.interact(|conn1| {
    //             conn1.transaction()
    //         }).await.map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {:?}", e)))?
    //             .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {:?}", e)))?;
    //
    //         // 使用 unsafe 扩展事务生命周期
    //         let tx = unsafe {
    //             std::mem::transmute::<rusqlite::Transaction<'_>, rusqlite::Transaction<'static>>(tx)
    //         };
    //
    //         // 存储事务
    //         let tx_ref = SQLITE_TX_REGISTER.try_get();
    //         if tx_ref.is_ok() {
    //             let tx_ref = tx_ref.unwrap().clone();
    //             let mut tx_guard = tx_ref.lock().await;
    //             *tx_guard = Some(tx);
    //             Ok(())
    //         } else {
    //             Err(DatabaseError::CommonError("Transaction register not found".to_string()))
    //         }
    //     } else {
    //         Err(DatabaseError::CommonError("Connection not found".to_string()))
    //     }
    // }
    //
    // async fn commit(&self) -> Result<(), DatabaseError> {
    //     let tx_ref = SQLITE_TX_REGISTER.try_get();
    //     if tx_ref.is_ok() {
    //         let tx_ref = tx_ref.unwrap().clone();
    //         let mut tx_guard = tx_ref.lock().await;
    //         if let Some(mut tx) = tx_guard.take() {
    //             tx.commit().map_err(|e| DatabaseError::CommonError(format!("Failed to commit transaction: {:?}", e)))?;
    //             Ok(())
    //         } else {
    //             Err(DatabaseError::CommonError("No active transaction".to_string()))
    //         }
    //     } else {
    //         Err(DatabaseError::CommonError("Transaction register not found".to_string()))
    //     }
    // }
    //
    // async fn rollback(&self) -> Result<(), DatabaseError> {
    //     let tx_ref = SQLITE_TX_REGISTER.try_get();
    //     if tx_ref.is_ok() {
    //         let tx_ref = tx_ref.unwrap().clone();
    //         let mut tx_guard = tx_ref.lock().await;
    //         if let Some(mut tx) = tx_guard.take() {
    //             tx.rollback().map_err(|e| DatabaseError::CommonError(format!("Failed to rollback transaction: {:?}", e)))?;
    //             Ok(())
    //         } else {
    //             Err(DatabaseError::CommonError("No active transaction".to_string()))
    //         }
    //     } else {
    //         Err(DatabaseError::CommonError("Transaction register not found".to_string()))
    //     }
    // }

    fn row_to_e<'a, E>(row: &Self::Row<'a>) -> Result<E, DatabaseError>
    where
        E: Entity
    {
            let mut e = E::new();
            for col in E::column_names() {
                let val = row.get_ref(col)?;
                let param_value = value_to_param_value(val)?;
                e.set_value_by_column_name(col, param_value);
            }
            Ok(e)
    }

    // async fn start_transaction(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn commit(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn rollback(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }

    // async fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    // where
    //     E: Entity,
    // {
    //     query_basic::<E, Vec<E>>(sql.to_string(), params.to_vec(), entity_mapper::<E>, |results: Vec<E>| {
    //         Ok(results)
    //     }).await
    // }
    //
    // async fn query_one<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    // where
    //     E: Entity,
    // {
    //     query_basic(sql.to_string(), params.to_vec(), entity_mapper::<E>, |results: Vec<E>| {
    //         Ok(results.into_iter().next())
    //     }).await
    // }
    //
    // async fn query_count(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
    //     query_basic(
    //         sql.to_string(),
    //         params.to_vec(),
    //         |row| {
    //             let v = row.get_ref(0)?;
    //             Ok(v.as_i64().unwrap())
    //         },
    //         |results: Vec<i64>| Ok(results[0] as u64),
    //     ).await
    // }
    //
    // async fn insert<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    // where
    //     E: Entity,
    // {
    //     query_basic(
    //         sql.to_string(),
    //         params.to_vec(),
    //         |row| {
    //             let val = row.get_ref(0)?;
    //             value_to_param_value(val)
    //         },
    //         |results: Vec<ParamValue>| {
    //             if results.is_empty() {
    //                 Ok(None)
    //             } else {
    //                 Ok(Some(results[0].clone().into()))
    //             }
    //         },
    //     ).await
    // }
    //
    // async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    // where
    //     E: Entity,
    // {
    //     exec_basic(sql.to_string(), params.clone()).await
    // }
    //
    // async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
    //     exec_basic(sql.to_string(), params.clone()).await
    // }
    //
    // async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
    //     exec_basic(sql.to_string(), params.clone()).await
    // }
    //
    // async fn start_transaction(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn commit(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn rollback(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
}

fn value_to_param_value(value: ValueRef<'_>) -> Result<ParamValue, DatabaseError> {
    let param_value;
    match value {
        ValueRef::Null => param_value = ParamValue::Null,
        ValueRef::Integer(v) => param_value = ParamValue::I64(v),
        ValueRef::Real(v) => param_value = ParamValue::F64(v),
        ValueRef::Text(v) => {
            let s = String::from_utf8(v.to_vec());
            match s {
                Ok(s) => param_value = ParamValue::String(s),
                Err(e) => {
                    return Err(DatabaseError::CommonError(format!("字符串转换异常: {}", e)));
                }
            }
        }
        ValueRef::Blob(v) => param_value = ParamValue::Blob(v.to_vec()),
    }
    Ok(param_value)
}

const fn make_e<E>() -> impl FnMut(&Row<'_>) -> Result<E, DatabaseError>
where
    E: Entity,
{
    |row| {
        let mut e = E::new();
        for col in E::column_names() {
            let val = row.get_ref(col).map_err(|e| DatabaseError::CommonError(format!("Failed to get column {}: {:?}", col, e)))?;
            let param_value = value_to_param_value(val)?;
            e.set_value_by_column_name(col, param_value);
        }
        Ok(e)
    }
}

// 将闭包改为函数指针形式
fn entity_mapper<'a, E>(row: &Row<'a>) -> Result<E, DatabaseError> where E: Entity {
    let mut e = E::new();
    for col in E::column_names() {
        let val = row.get_ref(col).map_err(|e| DatabaseError::CommonError(format!("Failed to get column {}: {:?}", col, e)))?;
        let param_value = value_to_param_value(val)?;
        e.set_value_by_column_name(col, param_value);
    }
    Ok(e)
}

pub fn to_sql(param_value: & ParamValue) -> & dyn ToSql {
    match param_value {
        // ParamValue::U64(x) => {let v = (*x as i64) ; let vx =  &v; vx as &dyn ToSql},
        ParamValue::U32(x) => x as &dyn ToSql,
        ParamValue::U16(x) => x as &dyn ToSql,
        ParamValue::U8(x) => x as &dyn ToSql,
        ParamValue::I64(x) => x as &dyn ToSql,
        ParamValue::I32(x) => x as &dyn ToSql,
        ParamValue::I16(x) => x as &dyn ToSql,
        ParamValue::I8(x) => x as &dyn ToSql,
        ParamValue::String(x) => x as &dyn ToSql,
        ParamValue::F32(x) => x as &dyn ToSql,
        ParamValue::F64(x) => x as &dyn ToSql,
        ParamValue::Bool(x) => x as &dyn ToSql,
        ParamValue::Blob(x) => x as &dyn ToSql,
        ParamValue::Clob(x) => x as &dyn ToSql,
        ParamValue::Null => &rusqlite::types::Null as &dyn ToSql,
        ParamValue::DateTime(_) => {
            // 暂时将 DateTime 转换为字符串，使用静态引用
            // 实际生产环境中可能需要更复杂的处理
            &rusqlite::types::Null as &dyn ToSql
        },
        _ => panic!("Unsupported parameter type"),
    }
}

async fn get_conn()->Object<Manager>{
    let p:Arc<DbManager<Pool>> = DbManager::get_instance().unwrap();
    let conn = p.get_pool().get().await.unwrap();
    conn
}