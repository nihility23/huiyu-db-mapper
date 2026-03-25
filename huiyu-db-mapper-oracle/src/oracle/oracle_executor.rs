use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::util::time_util;
use huiyu_db_mapper_core::with_conn_scope;
use std::sync::Arc;
use deadpool_oracle::{Object, Pool};
use oracle_rs::Value;
use tokio::sync::Mutex;
use tokio::task_local;

task_local! {
    pub static ORACLE_CONN_REGISTER : Arc<Mutex<Object>>;
}
#[derive(Clone)]
pub struct OracleSqlExecutor;
// 全局单例
pub const ORACLE_SQL_EXECUTOR: OracleSqlExecutor = OracleSqlExecutor;

pub struct OracleRow(oracle_rs::Row);

impl RowType for OracleRow {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.0.get(col_index);
        if val.is_none(){
            Ok(ParamValue::Null)
        }else{
            Ok(value_to_param_value(val.unwrap())?)
        }
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let val = self.0.get_by_name(col_name);
        if val.is_none(){
            Ok(ParamValue::Null)
        }else{
            Ok(value_to_param_value(val.unwrap())?)
        }
    }
}
// 查询基本实现
impl Executor for OracleSqlExecutor {
    type Row<'a> = OracleRow;
    type Conn = Object;


    async fn query<T, R, F, Q>(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        let sql = sql.to_string();
        let params = params.clone();
        let conn = conn.lock().await;
            let param_refs = ParamValueWrapper::convert_param_values(&params)?;
            let to_sql_values = param_refs.iter().map(|x| x.as_sql_param()).collect::<Result<Vec<_>, DatabaseError>>()?;

            let mut rows = conn.query(sql.as_str(),&*to_sql_values).await.map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute query: {:?}", e)))?;
            let mut results = Vec::new();
            for row in rows{
                results.push(mapper(&OracleRow(row)).map_err(|e| DatabaseError::RowConvertError(format!("Failed to map row: {:?}", e)))?);
            }
            processor(results)
        }

    async fn execute(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let sql = sql.to_string();
        let params = params.clone();
        let conn = conn.lock().await;
        let param_refs = ParamValueWrapper::convert_param_values(&params)?;
        let to_sql_values = param_refs.iter().map(|x| x.as_sql_param()).collect::<Result<Vec<_>, DatabaseError>>()?;
        let res = conn.execute(sql.as_str(), &*to_sql_values).await.map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute statement: {:?}", e)))?;
        let row = res.first().ok_or(DatabaseError::ExecuteError("No rows returned".to_string()))?;
        Ok(row.get(0).unwrap().as_i64().unwrap() as u64)
    }

    fn get_conn_ref(&self) -> Result<Arc<Mutex<Self::Conn>>, DatabaseError> {
        let c = ORACLE_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("ORACLE_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        let p:Arc<DbManager<Pool>> = DbManager::get_instance(get_datasource_name().as_str())?;
        let conn = p.get_pool().get().await.map_err(|e| DatabaseError::ConnectCanNotGetError(format!("Failed to get database connection: {:?}", e)))?;
        Ok(conn)
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.execute("BEGIN TRANSACTION", &[] as &[Value]).await.map_err(|e| DatabaseError::ExecuteError(format!("Failed to start transaction: {:?}", e)))?;
        Ok(())
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.execute("COMMIT", &[] as &[Value]).await.map_err(|e| DatabaseError::ExecuteError(format!("Failed to commit transaction: {:?}", e)))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.execute("ROLLBACK", &[] as &[Value]).await.map_err(|e| DatabaseError::ExecuteError(format!("Failed to rollback transaction: {:?}", e)))?;
        Ok(())
    }

    async fn transaction_basic_exec<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output=Result<T, DatabaseError>>
    {
        with_conn_scope!(ORACLE_CONN_REGISTER, self, func)
    }

}

struct ParamValueWrapper(ParamValue);

impl ParamValueWrapper {

    /***
        Convert ParamValue -> ParamValueWrapper， 自己扩展用，以便数据兼容性的扩展
     */
    fn convert_param_values(param_values: &Vec<ParamValue>) -> Result<Vec<ParamValueWrapper>,DatabaseError> {
        param_values.iter().map(|param_value: &ParamValue|{
            match param_value {
                ParamValue::U64(x) => Ok(ParamValueWrapper(ParamValue::I64(*x as i64))),
                ParamValue::U32(x) => Ok(ParamValueWrapper(ParamValue::U32(*x ))),
                ParamValue::U16(x) => Ok(ParamValueWrapper(ParamValue::U16(*x))),
                ParamValue::U8(x) => Ok(ParamValueWrapper(ParamValue::U8(*x)))        ,
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
                ParamValue::DateTime(x) => Ok(ParamValueWrapper(ParamValue::String(time_util::format_date_time_local(x, "%Y-%m-%d %H:%M:%S")))),
                _ => Err(DatabaseError::ConvertError(format!("Can't Convert Postgres Error: {:?}", param_value)))
            }
        }).collect()
    }

    /***
        ParamValue -> 数据库Value，查询组装参数用
     */
    fn as_sql_param(&self) -> Result<Value, DatabaseError> {
        match &self.0 {
            ParamValue::Null => Ok(Value::Null),
            ParamValue::I64(v) => Ok(Value::Integer(*v)),
            ParamValue::I32(v) => Ok(Value::Integer(*v as i64)),
            ParamValue::I16(v) => Ok(Value::Integer(*v as i64)),
            ParamValue::I8(v) => Ok(Value::Integer(*v as i64)),
            ParamValue::String(v) => Ok(Value::String(v.to_string())) ,
            ParamValue::F64(v) => Ok(Value::Float(*v))      ,
            ParamValue::F32(v) => Ok(Value::Float(*v as f64)),
            ParamValue::Bool(v) => Ok(Value::Boolean(*v)),
            ParamValue::Blob(v) => Ok(Value::Bytes(v.to_vec())),
            ParamValue::Clob(v) => Ok(Value::String(String::from_utf8(v.to_vec()).unwrap())),
            ParamValue::U32(v) => Ok(Value::Integer(*v as i64)),
            ParamValue::U16(v) => Ok(Value::Integer(*v as i64)),
            ParamValue::U8(v) => Ok(Value::Integer(*v as i64)),
            _ => Err(DatabaseError::ConvertError(format!("Can't Convert Oracle Error: {:?}", self.0)))
        }
    }

}

    /***
        数据库Value->ParamValue,查询返回用
     */
fn value_to_param_value(value: &Value) -> Result<ParamValue, DatabaseError> {
    let param_value;
    match value {
        Value::Null => param_value = ParamValue::Null,
        Value::String(v) => param_value = ParamValue::String(v.to_string()),
        Value::Bytes(v) => param_value = ParamValue::Blob(v.to_vec()),
        Value::Integer(v) => param_value = ParamValue::I64(*v),
        Value::Float(v) => param_value = ParamValue::F64(*v),
        Value::Number(v) => param_value = ParamValue::F64(v.to_f64().unwrap()),
        Value::Boolean(v) => param_value = ParamValue::Bool(*v),
        Value::Date(v) => param_value = ParamValue::DateTime(time_util::create_datetime_local(v.year, v.month as u32, v.day as u32, v.hour as u32, v.minute as u32, v.second as u32, 0)),
        Value::Timestamp(v) => param_value = ParamValue::DateTime(time_util::create_datetime_local(v.year, v.month as u32, v.day as u32, v.hour as u32, v.minute as u32, v.second as u32, 0)),
        _ => return Err(DatabaseError::ConvertError(format!("Can't Convert Oracle Error: {:?}", value)))
    }
    Ok(param_value)
}
