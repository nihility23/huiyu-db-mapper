use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::error::DatabaseError::CommonError;
use crate::base::param::ParamValue;
use crate::pool::db_manager::DbManager;
use crate::sql::executor::Executor;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql, Transaction};
use std::cell::RefCell;
use std::rc::Rc;
use tokio::sync::Mutex;
use std::sync::Arc;
use deadpool::managed::Object;
use deadpool_sqlite::{Manager, Pool};
use rustlog::info;
use tokio::task::spawn_blocking;
use tokio::task_local;

task_local! {
    pub static SQLITE_CONN_REGISTER : Arc<Mutex<Object<Manager>>>;
}
#[derive(Clone)]
pub struct SqliteSqlExecutor;
// 全局单例
pub const SQLITE_SQL_EXECUTOR: SqliteSqlExecutor = SqliteSqlExecutor;
// 提取公共函数避免重复
async fn query<T, R>(
    conn: &deadpool_sync::SyncWrapper<rusqlite::Connection>,
    sql: String,
    params: Vec<ParamValue>,
    f: fn(&Row) -> rusqlite::Result<T>,
    q: fn(Vec<T>) -> Result<R, DatabaseError>,
) -> Result<R, DatabaseError> where R: Send + 'static, T: Send + 'static
{
    conn.interact(move |conn| {
        let mut stmt = conn.prepare(sql.as_str())?;
        let param_refs: Vec<&dyn ToSql> = params.iter().map(|x| to_sql(x)).collect();

        let rows = stmt.query_map(&*param_refs, f)?;
        let mut results = Vec::new();

        for row in rows {
            results.push(row?);
        }

        q(results)
    }).await.unwrap()

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
    }).await.unwrap()

}


// 查询基本实现
async fn query_basic<T, R>(
    sql: String,
    params: Vec<ParamValue>,
    f: fn(&Row) -> rusqlite::Result<T>,
    q: fn(Vec<T>) -> Result<R, DatabaseError>,
) -> Result<R, DatabaseError> where T: Send + 'static, R:Send + 'static{

        let conn_ref = SQLITE_CONN_REGISTER.try_get();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            let conn = conn_ref.lock().await;
            let conn = conn.as_ref();
            query(conn, sql, params,f,q).await // 现在可以借用
        } else {
            let conn:Object<Manager> = get_conn().await;
            query(conn.as_ref(), sql, params,f,q).await
        }

}

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

impl Executor for SqliteSqlExecutor {
    async fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity,
    {
        query_basic::<E, Vec<E>>(sql.to_string(), params.to_vec(), entity_mapper::<E>, |results: Vec<E>| {
            Ok(results)
        }).await
    }

    async fn query_one<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity,
    {
        query_basic(sql.to_string(), params.to_vec(), entity_mapper::<E>, |results: Vec<E>| {
            Ok(results.into_iter().next())
        }).await
    }

    async fn query_count(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        query_basic(
            sql.to_string(),
            params.to_vec(),
            |row| {
                let v = row.get_ref(0)?;
                Ok(v.as_i64().unwrap())
            },
            |results: Vec<i64>| Ok(results[0] as u64),
        ).await
    }

    async fn insert<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity,
    {
        query_basic(
            sql.to_string(),
            params.to_vec(),
            |row| {
                let val = row.get_ref(0)?;
                value_to_param_value(val)
            },
            |results: Vec<ParamValue>| {
                if results.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(results[0].clone().into()))
                }
            },
        ).await
    }

    async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity,
    {
        exec_basic(sql.to_string(), params.clone()).await
    }

    async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic(sql.to_string(), params.clone()).await
    }

    async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic(sql.to_string(), params.clone()).await
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        todo!()
    }
}

fn value_to_param_value(value: ValueRef<'_>) -> Result<ParamValue, Error> {
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
                    return Err(Error::FromSqlConversionFailure(
                        0,
                        Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("字符串转换异常: {}", e)
                        )),
                    ));
                }
            }
        }
        ValueRef::Blob(v) => param_value = ParamValue::Blob(v.to_vec()),
    }
    Ok(param_value)
}

const fn make_e<E>() -> impl FnMut(&Row<'_>) -> rusqlite::Result<E>
where
    E: Entity,
{
    |row| {
        let mut e = E::new();
        for col in E::column_names() {
            let val = row.get_ref(col)?;
            let param_value = value_to_param_value(val)?;
            e.set_value_by_column_name(col, param_value);
        }
        Ok(e)
    }
}

// 将闭包改为函数指针形式
fn entity_mapper<E: Entity>(row: &Row) -> rusqlite::Result<E> {
    let mut e = E::new();
    for col in E::column_names() {
        let val = row.get_ref(col)?;
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
        ParamValue::DateTime(_) => &rusqlite::types::Null as &dyn ToSql,
        _ => panic!("Unsupported parameter type"),
    }
}

async fn get_conn()->Object<Manager>{
    let p:Arc<DbManager<Pool>> = DbManager::get_instance().unwrap();
    let conn = p.get_pool().get().await.unwrap();
    conn
}