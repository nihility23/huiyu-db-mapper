use deadpool_sqlite::{Manager, Object, Pool};
use rusqlite::types::ValueRef;
use rusqlite::{Row, ToSql};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task_local;
use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};

task_local! {
    pub static SQLITE_CONN_REGISTER : Arc<Mutex<Object>>;
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
        let mut stmt = conn.prepare(sql.as_str()).map_err(|e| DatabaseError::CommonError(format!("Failed to prepare statement: {:?}", e)))?;
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();

        let mut rows = stmt.query(&*param_refs).map_err(|e| DatabaseError::CommonError(format!("Failed to execute query: {:?}", e)))?;
        let mut results = Vec::new();
        while let Some(row) = rows.next().map_err(|e| DatabaseError::CommonError(format!("Failed to fetch row: {:?}", e)))? {
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
        let res = conn.execute(sql.as_str(), &*param_refs).map_err(|e| DatabaseError::CommonError(format!("Failed to execute statement: {:?}", e)))?;
        Ok(res as u64)
    }).await.map_err(|e| DatabaseError::CommonError(format!("Database interaction failed: {:?}", e)))?

}

pub struct SqliteRow<'a>(&'a rusqlite::Row<'a>);

impl<'a> RowType for SqliteRow<'a> {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.0.get_ref(col_index).map_err(|e| DatabaseError::CommonError(format!("Failed to get column value: {:?}", e)))?;
        Ok(value_to_param_value(val)?)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.0.get_ref(col_name).map_err(|e| DatabaseError::CommonError(format!("Failed to get column value: {:?}", e)))?;
        Ok(value_to_param_value(val)?)
    }
}

// 查询基本实现

impl Executor for SqliteSqlExecutor {
    type Row<'a> = SqliteRow<'a>;
    type Conn = Object;
    type ConnWrapper = deadpool_sync::SyncWrapper<rusqlite::Connection>;
    

    async fn query<T, R, F, Q>(&self, conn: &Self::ConnWrapper, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        let sql = sql.to_string();
        let params = params.clone();
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(sql.as_str()).map_err(|e| DatabaseError::CommonError(format!("Failed to prepare statement: {:?}", e)))?;
            let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();

            let mut rows = stmt.query(&*param_refs).map_err(|e| DatabaseError::CommonError(format!("Failed to execute query: {:?}", e)))?;
            let mut results = Vec::new();

            while let Some(row) = rows.next().map_err(|e| DatabaseError::CommonError(format!("Failed to fetch row: {:?}", e)))? {
                results.push(mapper(&SqliteRow(row)).map_err(|e| DatabaseError::CommonError(format!("Failed to map row: {:?}", e)))?);
            }

            processor(results)
        }).await.map_err(|e| DatabaseError::CommonError(format!("Database interaction failed: {:?}", e)))?
    }

    async fn execute(&self, conn: &Self::ConnWrapper, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let sql = sql.to_string();
        let params = params.clone();
        conn.interact(move |conn| {
            let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();
            let res = conn.execute(sql.as_str(), &*param_refs).map_err(|e| DatabaseError::CommonError(format!("Failed to execute statement: {:?}", e)))?;
            Ok(res as u64)
        }).await.map_err(|e| DatabaseError::CommonError(format!("Database interaction failed: {:?}", e)))?
    }

    fn get_conn_ref(&self) -> Result<Arc<Mutex<Self::Conn>>, DatabaseError> {
        let c = SQLITE_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("SQLITE_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Self::Conn {
        let p:Arc<DbManager<Pool>> = DbManager::get_instance(get_datasource_name().as_str()).unwrap();
        let conn = p.get_pool().get().await.unwrap();
        conn
    }
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

// async fn get_conn()->Object{
//     let p:Arc<DbManager<Pool>> = DbManager::get_instance().unwrap();
//     let conn = p.get_pool().get().await.unwrap();
//     conn
// }