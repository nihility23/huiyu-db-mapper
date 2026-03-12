
use deadpool_postgres::{ClientWrapper, Object, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task_local;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Row;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};

task_local! {
    pub static POSTGRES_CONN_REGISTER : Arc<Mutex<Object>>;
}
#[derive(Clone)]
pub struct PostgresSqlExecutor;
// 全局单例
pub const POSTGRES_SQL_EXECUTOR: PostgresSqlExecutor = PostgresSqlExecutor;

pub struct PostgresRow{
    row: Row
}

impl RowType for PostgresRow {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.row.get::<usize, PostgresParamValue>(col_index);
        Ok(v.0)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.row.get::<&str, PostgresParamValue>(col_name);
        Ok(v.0)
    }
}

impl Executor for PostgresSqlExecutor {

    type Row<'a> = PostgresRow;
    type Conn = Object;
    // type ConnWrapper = ClientWrapper;


    async fn query<T, R, F, Q>(&self, conn: &mut Self::Conn, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {

        let mut str = sql.to_string();
        for i in 0..params.len() {
            str = str.replacen("?", &format!("${}", i+1), 1);
            println!("{}", str);
        }
        let stmt = conn.prepare(str.as_str()).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect::<Result<_, _>>()?;

        let results = conn.query(&stmt, &param_refs).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?.iter().map(|row| mapper(&PostgresRow{row: row.clone()})).collect::<Result<Vec<_>, _>>()?;

        processor(results)
    }

    async fn execute(&self, conn: &mut Self::Conn, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let mut str = sql.to_string();
        for i in 0..params.len() {
            str = str.replacen("?", &format!("${}", i+1), 1);
            println!("{}", str);
        }
        let param_refs: Vec<&(dyn ToSql+Sync)> = params.iter().map(|x| to_sql(x)).collect::<Result<_, _>>()?;
        let res = conn.execute(str.as_str(), &*param_refs).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        Ok(res as u64)
    }


    fn get_conn_ref(&self)-> Result<Arc<Mutex<Object>>,DatabaseError> {
        let c = POSTGRES_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("POSTGRES_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        DbManager::<Pool>::get_instance(get_datasource_name().as_str()).unwrap().get_pool().get().await.map_err(|e| DatabaseError::ConnectCanNotGetError(e.to_string()))
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

pub struct PostgresParamValue(ParamValue);
impl FromSql<'_> for PostgresParamValue {
    fn from_sql(ty: &tokio_postgres::types::Type, _bytes: &[u8]) -> Result<PostgresParamValue, Box<(dyn std::error::Error + Send + Sync + 'static)>> {
        match ty {
            &Type::INT8 => Ok(PostgresParamValue(ParamValue::I64(i64::from_sql(ty, _bytes)?))),
            &Type::INT4 => Ok(PostgresParamValue(ParamValue::I32(i32::from_sql(ty, _bytes)?))),
            &Type::INT2 => Ok(PostgresParamValue(ParamValue::I16(i16::from_sql(ty, _bytes)?))),
            &Type::BOOL => Ok(PostgresParamValue(ParamValue::Bool(bool::from_sql(ty, _bytes)?))),
            &Type::TEXT => Ok(PostgresParamValue(ParamValue::String(String::from_sql(ty, _bytes)?))),
            &Type::VARCHAR => Ok(PostgresParamValue(ParamValue::String(String::from_sql(ty, _bytes)?))),
            _ => Err(Box::new(DatabaseError::ConvertError(format!("Unsupported type {}", ty.name())))),
        }
    }

    fn accepts(ty: &Type) -> bool {
        true
    }
}