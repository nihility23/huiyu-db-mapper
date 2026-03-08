use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::pool::db_manager::DbManager;
use crate::sql::executor::Executor;
use deadpool_postgres::{ClientWrapper, Object, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task_local;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Row;

task_local! {
    pub static POSTGRES_CONN_REGISTER : Arc<Mutex<Object>>;
}
#[derive(Clone)]
pub struct PostgresSqlExecutor;
// 全局单例
pub const POSTGRES_SQL_EXECUTOR: PostgresSqlExecutor = PostgresSqlExecutor;
// 提取公共函数避免重复
async fn query<T, R>(
    conn: &ClientWrapper,
    sql: String,
    params: Vec<ParamValue>,
    f: fn(&Row) -> Result<T,DatabaseError>,
    q: fn(Vec<T>) -> Result<R, DatabaseError>,
) -> Result<R, DatabaseError> where R: Send + 'static, T: Send + 'static
{
        let mut stmt = conn.prepare(sql.as_str()).await?;
        let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect();
        // let mut param_refs: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        // for param in params {
        //     param_refs.push(param.clone() as &(dyn tokio_postgres::types::ToSql + Sync));
        // }

        let results = conn.query(&stmt, &param_refs).await?.iter().map(|row| f(row)).collect::<Result<Vec<_>, _>>()?;

        // let mut results = Vec::new();
        // for row in rows {
        //     results.push(row);
        // }

        q(results)

}

async fn execute(
    conn: &ClientWrapper,
    sql: String,
    params: Vec<ParamValue>,
) -> Result<u64, DatabaseError>{
    let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect();
    let res = conn.execute(sql.as_str(), &*param_refs).await?;
    Ok(res as u64)
}


// 查询基本实现
async fn query_basic<T, R>(
    sql: String,
    params: Vec<ParamValue>,
    f: fn(&Row) -> Result<T,DatabaseError>,
    q: fn(Vec<T>) -> Result<R, DatabaseError>,
) -> Result<R, DatabaseError> where T: Send + 'static, R:Send + 'static{

        let conn_ref = POSTGRES_CONN_REGISTER.try_get();
        // if conn_ref.is_ok() {
        //     let conn_ref = conn_ref.unwrap().clone();
        //     let conn = conn_ref.lock().await;
        //     let conn = conn.as_ref();
        //     query(conn, sql, params,f,q).await // 现在可以借用
        // } else {
            let conn = get_conn().await;
            query(&conn, sql, params,f,q).await
        // }

}

async fn exec_basic(sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
    let conn_ref = POSTGRES_CONN_REGISTER.try_get();
    if conn_ref.is_ok() {
        let conn_ref = conn_ref.unwrap().clone();
        let conn = conn_ref.lock().await;
        let conn = conn.as_ref();
        execute(conn, sql, params).await
    } else {
        let conn:Object = get_conn().await;
        execute(conn.as_ref(), sql, params).await
    }


}

impl Executor for PostgresSqlExecutor {
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
                let v:i64 = row.get::<usize, i64>(0);
                Ok(v)
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
                let val:ParamValue = row.get::<usize, ParamValue>(0);
                Ok(val)
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


const fn make_e<E>() -> impl FnMut(&Row) -> rusqlite::Result<E>
where
    E: Entity,
{
    |row| {
        let mut e = E::new();
        for col in E::column_names() {
            let val:ParamValue = row.get(col);
            e.set_value_by_column_name(col, val);
        }
        Ok(e)
    }
}

// 将闭包改为函数指针形式
fn entity_mapper<E: Entity>(row: &Row) -> Result<E,DatabaseError> {
    let mut e = E::new();
    for col in E::column_names() {
        let val:ParamValue = row.get(col);
        e.set_value_by_column_name(col, val);
    }
    Ok(e)
}

pub fn to_sql(param_value: & ParamValue) -> & (dyn ToSql +Sync) {
    match param_value {
        // ParamValue::U64(x) => {let v = (*x as i64) ; let vx =  &v; vx as &dyn ToSql},
        ParamValue::U32(x) => x as &(dyn ToSql+Sync),
        // ParamValue::U16(x) => x as &dyn ToSql,
        // ParamValue::U8(x) => x as &dyn ToSql,
        ParamValue::I64(x) => x as &(dyn ToSql+Sync),
        ParamValue::I32(x) => x as &(dyn ToSql+Sync),
        ParamValue::I16(x) => x as &(dyn ToSql+Sync),
        ParamValue::I8(x) => x as &(dyn ToSql+Sync),
        ParamValue::String(x) => x as &(dyn ToSql+Sync),
        ParamValue::F32(x) => x as &(dyn ToSql+Sync),
        ParamValue::F64(x) => x as &(dyn ToSql+Sync),
        ParamValue::Bool(x) => x as &(dyn ToSql+Sync),
        ParamValue::Blob(x) => x as &(dyn ToSql+Sync),
        ParamValue::Clob(x) => x as &(dyn ToSql+Sync),
        // ParamValue::Null => &tokio_postgres::types::WasNull as &dyn ToSql,
        // ParamValue::DateTime(x) => x as &dyn ToSql,
        _ => panic!("Unsupported parameter type"),
    }
}

async fn get_conn()-> Object {
    let p:Arc<DbManager<Pool>> = DbManager::get_instance().unwrap();
    let conn = p.get_pool().get().await.unwrap();
    conn
}

impl FromSql<'_> for ParamValue {
    fn from_sql(ty: &tokio_postgres::types::Type, _bytes: &[u8]) -> Result<ParamValue, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        match ty {
            &Type::INT8 => Ok(ParamValue::I64(i64::from_sql(ty, _bytes)?)),
            &Type::INT4 => Ok(ParamValue::I32(i32::from_sql(ty, _bytes)?)),
            &Type::INT2 => Ok(ParamValue::I16(i16::from_sql(ty, _bytes)?)),
            &Type::BOOL => Ok(ParamValue::Bool(bool::from_sql(ty, _bytes)?)),
            _ => panic!("Unsupported type"),
        }
    }

    fn accepts(ty: &Type) -> bool {
        todo!()
    }
}