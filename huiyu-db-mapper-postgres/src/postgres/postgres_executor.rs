use deadpool_postgres::{Object, Pool};
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use std::sync::{Arc, Mutex};
use tokio::task_local;
use tokio_postgres::types::{FromSql, ToSql, Type};
use tokio_postgres::Row;
use huiyu_db_mapper_core::with_conn_scope;

task_local! {
    pub static POSTGRES_CONN_REGISTER : Arc<std::sync::Mutex<Object>>;
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
        let v = self.row.get::<usize, ParamValueWrapper>(col_index);
        Ok(v.0)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.row.get::<&str, ParamValueWrapper>(col_name);
        Ok(v.0)
    }
}

impl Executor for PostgresSqlExecutor {

    type Row<'a> = PostgresRow;
    type Conn = Object;


    async fn query<T, R, F, Q>(&self, conn: Arc<std::sync::Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
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
        let stmt = conn.lock().unwrap().prepare(str.as_str()).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        let sql_values = ParamValueWrapper::convert_param_values(params)?;
        // 获取引用
        let param_refs: Vec<&(dyn ToSql + Sync)> = sql_values
            .iter()
            .map(|v| v.as_sql_param())
            .collect();
        let results = conn.lock().unwrap().query(&stmt, &param_refs).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?.iter().map(|row| mapper(&PostgresRow{row: row.clone()})).collect::<Result<Vec<_>, _>>()?;

        processor(results)
    }

    async fn execute(&self, conn: Arc<std::sync::Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let mut str = sql.to_string();
        for i in 0..params.len() {
            str = str.replacen("?", &format!("${}", i+1), 1);
            println!("{}", str);
        }
        let sql_values = ParamValueWrapper::convert_param_values(params)?;

        // 获取引用
        let param_refs: Vec<&(dyn ToSql + Sync)> = sql_values
            .iter()
            .map(|v| v.as_sql_param())
            .collect();

        let res = conn.lock().unwrap().execute(str.as_str(), &*param_refs).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        Ok(res as u64)
    }

    fn get_conn_ref(&self)-> Result<Arc<std::sync::Mutex<Object>>,DatabaseError> {
        let c = POSTGRES_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("POSTGRES_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        DbManager::<Pool>::get_instance(get_datasource_name().as_str()).unwrap().get_pool().get().await.map_err(|e| DatabaseError::ConnectCanNotGetError(e.to_string()))
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        conn.lock().map_err(|e| DatabaseError::ExecuteError(e.to_string()))?.execute("BEGIN", &[]).await.map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        conn.lock().map_err(|e| DatabaseError::ExecuteError(e.to_string()))?.execute("COMMIT", &[]).await.map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        conn.lock().map_err(|e| DatabaseError::ExecuteError(e.to_string()))?.execute("ROLLBACK", &[]).await.map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn transaction_exec<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output=Result<T, DatabaseError>>
    {
        with_conn_scope!(POSTGRES_CONN_REGISTER,self,func)
    }
}


/***
    每个数据库类型支持不同
 */
struct ParamValueWrapper(ParamValue);

/***
    处理数据库查询结果转换
 */
impl FromSql<'_> for ParamValueWrapper {
    fn from_sql(ty: &Type, _bytes: &[u8]) -> Result<ParamValueWrapper, Box<dyn std::error::Error + Send + Sync + 'static>> {
        match ty {
            &Type::INT8 => Ok(ParamValueWrapper(ParamValue::I64(i64::from_sql(ty, _bytes)?))),
            &Type::INT4 => Ok(ParamValueWrapper(ParamValue::I32(i32::from_sql(ty, _bytes)?))),
            &Type::INT2 => Ok(ParamValueWrapper(ParamValue::I16(i16::from_sql(ty, _bytes)?))),
            &Type::BOOL => Ok(ParamValueWrapper(ParamValue::Bool(bool::from_sql(ty, _bytes)?))),
            &Type::TEXT => Ok(ParamValueWrapper(ParamValue::String(String::from_sql(ty, _bytes)?))),
            &Type::VARCHAR => Ok(ParamValueWrapper(ParamValue::String(String::from_sql(ty, _bytes)?))),
            &Type::FLOAT4 => Ok(ParamValueWrapper(ParamValue::F32(f32::from_sql(ty, _bytes)?))),
            &Type::FLOAT8 => Ok(ParamValueWrapper(ParamValue::F64(f64::from_sql(ty, _bytes)?))),
            &Type::BYTEA => Ok(ParamValueWrapper(ParamValue::Blob(Vec::<u8>::from_sql(ty, _bytes)?))),
            &Type::DATE => {
                Ok(ParamValueWrapper(ParamValue::DateTime(chrono::DateTime::<chrono::Local>::from_sql(ty, _bytes)?)))
            },
            &Type::TIMESTAMP => {
                Ok(ParamValueWrapper(ParamValue::DateTime(chrono::DateTime::<chrono::Local>::from_sql(ty, _bytes)?)))
            },
            _ => Err(Box::new(DatabaseError::ConvertError(format!("Unsupported type {}", ty.name())))),
        }
    }

    fn accepts(_: &Type) -> bool {
        true
    }
}

/**
    ParamValueWrapper
    处理查询参数转换
 */
impl ParamValueWrapper {
    fn convert_param_values(param_values: &Vec<ParamValue>) -> Result<Vec<ParamValueWrapper>,DatabaseError> {
        param_values.iter().map(|param_value: &ParamValue|{
            match param_value {
                ParamValue::U64(x) => Ok(ParamValueWrapper(ParamValue::I64(*x as i64))),
                ParamValue::U32(x) => Ok(ParamValueWrapper(ParamValue::I32(*x as i32))),
                ParamValue::U16(x) => Ok(ParamValueWrapper(ParamValue::I16(*x as i16))),
                ParamValue::U8(x) => Ok(ParamValueWrapper(ParamValue::I8(*x as i8)))        ,
                ParamValue::I64(x) => Ok(ParamValueWrapper(ParamValue::I64(*x))),
                ParamValue::I32(x) => Ok(ParamValueWrapper(ParamValue::I32(*x))),
                ParamValue::I16(x) => Ok(ParamValueWrapper(ParamValue::I16(*x))),
                ParamValue::I8(x) => Ok(ParamValueWrapper(ParamValue::I8(*x))),
                ParamValue::String(x) => Ok(ParamValueWrapper(ParamValue::String(x.to_string()))),
                ParamValue::F32(x) => Ok(ParamValueWrapper(ParamValue::F32(*x))),
                ParamValue::F64(x) => Ok(ParamValueWrapper(ParamValue::F64(*x))),
                ParamValue::Bool(x) => Ok(ParamValueWrapper(ParamValue::Bool(*x))),
                ParamValue::Blob(x) => Ok(ParamValueWrapper(ParamValue::Blob(x.to_vec()))),
                ParamValue::Clob(x) => Ok(ParamValueWrapper(ParamValue::String(String::from_utf8(x.to_vec()).unwrap()))),
                ParamValue::DateTime(x) => Ok(ParamValueWrapper(ParamValue::DateTime(*x))),
                _ => Err(DatabaseError::ConvertError(format!("Can't Convert Postgres Error: {:?}", param_value)))
            }
        }).collect()
    }

    pub fn as_sql_param(&self) -> &(dyn ToSql + Sync) {
        match &self.0 {
            ParamValue::I64(v) => v,
            ParamValue::I32(v) => v,
            ParamValue::I16(v) => v,
            ParamValue::I8(v) => v,
            ParamValue::String(v) => v,
            ParamValue::F32(v) => v,
            ParamValue::F64(v) => v,
            ParamValue::Bool(v) => v,
            ParamValue::Blob(v) => v,
            ParamValue::Clob(v) => v,
            ParamValue::DateTime(v) => v,
            _=> panic!()
        }
    }
}
