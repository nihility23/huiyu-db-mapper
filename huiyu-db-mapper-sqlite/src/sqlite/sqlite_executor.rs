use deadpool_sqlite::{Object, Pool};
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::util::time_util;
use huiyu_db_mapper_core::with_conn_scope;
use rusqlite::types::ValueRef;
use rusqlite::ToSql;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task_local;

task_local! {
    pub static SQLITE_CONN_REGISTER : Arc<Mutex<Object>>;
}
#[derive(Clone)]
pub struct SqliteSqlExecutor;
// 全局单例
pub const SQLITE_SQL_EXECUTOR: SqliteSqlExecutor = SqliteSqlExecutor;

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
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(sql.as_str()).map_err(|e| DatabaseError::ExecuteError(format!("Failed to prepare statement: {:?}", e)))?;
            let param_refs = ParamValueWrapper::convert_param_values(&params)?;
            let to_sql_values = param_refs.iter().map(|x| x.as_sql_param()).collect::<Result<Vec<_>, DatabaseError>>()?;

            let mut rows = stmt.query(&*to_sql_values).map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute query: {:?}", e)))?;
            let mut results = Vec::new();

            while let Some(row) = rows.next().map_err(|e| DatabaseError::RowConvertError(format!("Failed to fetch row: {:?}", e)))? {
                results.push(mapper(&SqliteRow(row)).map_err(|e| DatabaseError::RowConvertError(format!("Failed to map row: {:?}", e)))?);
            }

            processor(results)
        }).await.map_err(|e| DatabaseError::ExecuteError(format!("Database interaction failed: {:?}", e)))?
    }

    async fn execute(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let sql = sql.to_string();
        let params = params.clone();
        let conn = conn.lock().await;
        conn.interact(move |conn| {
            let param_refs = ParamValueWrapper::convert_param_values(&params)?;
            let to_sql_values = param_refs.iter().map(|x| x.as_sql_param()).collect::<Result<Vec<_>, DatabaseError>>()?;
            let res = conn.execute(sql.as_str(), &*to_sql_values).map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute statement: {:?}", e)))?;
            Ok(res as u64)
        }).await.map_err(|e| DatabaseError::ExecuteError(format!("Database interaction failed: {:?}", e)))?
    }

    fn get_conn_ref(&self) -> Result<Arc<Mutex<Self::Conn>>, DatabaseError> {
        let c = SQLITE_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("SQLITE_CONN_REGISTER is not set".to_string()));
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
        conn.interact(move |conn| {
            conn.execute("BEGIN TRANSACTION", []).map_err(|e| DatabaseError::ExecuteError(format!("Failed to start transaction: {:?}", e)))
        }).await.map_err(|e| DatabaseError::AccessError(format!("Failed to lock database connection: {:?}", e)))??;
        Ok(())
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.interact(move |conn| {
            conn.execute("COMMIT", []).map_err(|e| DatabaseError::ExecuteError(format!("Failed to start transaction: {:?}", e)))
        }).await.map_err(|e| DatabaseError::AccessError(format!("Failed to lock database connection: {:?}", e)))??;
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.interact(move |conn| {
            conn.execute("ROLLBACK", []).map_err(|e| DatabaseError::ExecuteError(format!("Failed to rollback transaction: {:?}", e)))
        }).await.map_err(|e| DatabaseError::AccessError(format!("Failed to lock database connection: {:?}", e)))??;
        Ok(())
    }

    // async fn transactional_exec<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    // where
    //     F: FnOnce() -> Fut ,  // BF 返回 Future
    //     Fut: Future<Output = Result<T, DatabaseError>>,
    // {
    //     // let conn = self.get_conn().await?;
    //     // let res = SQLITE_CONN_REGISTER.scope(Arc::new(Mutex::new(conn)), async {
    //     //     self.transaction_exec_basic(func).await
    //     // }).await;
    //     // res
    //     with_conn_scope!(SQLITE_CONN_REGISTER, self, func)
    // }
}

struct ParamValueWrapper(ParamValue);

impl ParamValueWrapper {

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
    fn as_sql_param(&self) -> Result<&dyn ToSql, DatabaseError> {
        match &self.0 {
            ParamValue::Null => Ok(&rusqlite::types::Null),
            ParamValue::I64(v) => Ok(v),
            ParamValue::I32(v) => Ok(v),
            ParamValue::I16(v) => Ok(v),
            ParamValue::I8(v) => Ok(v),
            ParamValue::String(v) => Ok(v ),
            ParamValue::F64(v) => Ok(v )      ,
            ParamValue::F32(v) => Ok(v),
            ParamValue::Bool(v) => Ok(v),
            ParamValue::Blob(v) => Ok(v),
            ParamValue::Clob(v) => Ok(v),
            ParamValue::U32(v) => Ok(v),
            ParamValue::U16(v) => Ok(v),
            ParamValue::U8(v) => Ok(v),
            _ => Err(DatabaseError::ConvertError(format!("Can't Convert Sqlite Error: {:?}", self.0)))
        }
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