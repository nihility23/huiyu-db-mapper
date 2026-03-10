use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::pool::db_manager::DbManager;
use crate::sql::executor::{Executor, RowType};
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
        let stmt = conn.prepare(sql.as_str()).await?;
        let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect::<Result<_, _>>()?;

        let results = conn.query(&stmt, &param_refs).await?.iter().map(|row| f(row)).collect::<Result<Vec<_>, _>>()?;

        q(results)

}

async fn execute(
    conn: &ClientWrapper,
    sql: String,
    params: Vec<ParamValue>,
) -> Result<u64, DatabaseError>{
    let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect::<Result<_, _>>()?;
    let res = conn.execute(sql.as_str(), &*param_refs).await?;
    Ok(res as u64)
}


// 查询基本实现
// async fn query_basic<T, R>(
//     sql: String,
//     params: Vec<ParamValue>,
//     f: fn(&Row) -> Result<T,DatabaseError>,
//     q: fn(Vec<T>) -> Result<R, DatabaseError>,
// ) -> Result<R, DatabaseError> where T: Send + 'static, R:Send + 'static{
//
//         let conn_ref = get_conn_ref();
//         if conn_ref.is_ok() {
//             let conn_ref = conn_ref.unwrap().clone();
//             let conn = conn_ref.lock().await;
//             let conn = conn.as_ref();
//             query(conn, sql, params,f,q).await // 现在可以借用
//         } else {
//             let conn = get_conn().await;
//             query(&conn, sql, params,f,q).await
//         }
//
// }
//
// async fn exec_basic(sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
//     let conn_ref = get_conn_ref();
//     if conn_ref.is_ok() {
//         let conn_ref = conn_ref.unwrap().clone();
//         let conn = conn_ref.lock().await;
//         let conn = conn.as_ref();
//         execute(conn, sql, params).await
//     } else {
//         let conn:Object = get_conn().await;
//         execute(conn.as_ref(), sql, params).await
//     }
// }

impl RowType for Row {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.get::<usize, ParamValue>(col_index);
        Ok(v)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.get::<&str, ParamValue>(col_name);
        Ok(v)
    }
}

impl Executor for PostgresSqlExecutor {

    type Row<'a> = Row;
    type Conn = Object;
    type ConnWrapper = ClientWrapper;


    async fn query<T, R, F, Q>(&self, conn: &Self::ConnWrapper, sql: String, params: Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        todo!()
    }

    async fn execute(&self, conn: &Self::ConnWrapper, sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }


    fn row_to_e<'a, E>(row: &Self::Row<'a>) -> Result<E, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn get_conn_ref(&self)-> Result<Arc<Mutex<Object>>,DatabaseError> {
        let c = POSTGRES_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("POSTGRES_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Self::Conn {
        DbManager::<Pool>::get_instance().unwrap().get_pool().get().await.unwrap()
    }
}



pub fn to_sql(param_value: & ParamValue) -> Result<&(dyn ToSql +Sync), DatabaseError> {
    match param_value {
        // ParamValue::U64(x) => {let v = (*x as i64) ; let vx =  &v; vx as &dyn ToSql},
        ParamValue::U32(x) => Ok(x as &(dyn ToSql+Sync)),
        // ParamValue::U16(x) => x as &dyn ToSql,
        // ParamValue::U8(x) => x as &dyn ToSql,
        ParamValue::I64(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::I32(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::I16(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::I8(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::String(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::F32(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::F64(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::Bool(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::Blob(x) => Ok(x as &(dyn ToSql+Sync)),
        ParamValue::Clob(x) => Ok(x as &(dyn ToSql+Sync)),
        // ParamValue::Null => &tokio_postgres::types::WasNull as &dyn ToSql,
        // ParamValue::DateTime(x) => x as &dyn ToSql,
        _ => Err(DatabaseError::ConvertError("Unsupported parameter type".to_string())),
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
            _ => Err(Box::new(DatabaseError::ConvertError(format!("Unsupported type {}", ty.name())))),
        }
    }

    fn accepts(ty: &Type) -> bool {
        todo!()
    }
}