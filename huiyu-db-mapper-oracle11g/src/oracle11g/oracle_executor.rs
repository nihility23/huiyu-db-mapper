use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::util::time_util;
use huiyu_db_mapper_core::with_conn_scope;
use std::sync::Arc;
use oracle::sql_type::{FromSql, OracleType, Timestamp, ToSql};
use oracle::SqlValue;
use r2d2::{Pool, PooledConnection};
use r2d2_oracle::OracleConnectionManager;
use tokio::sync::Mutex;
use tokio::task_local;

task_local! {
    pub static ORACLE11G_CONN_REGISTER : Arc<Mutex<PooledConnection<OracleConnectionManager>>>;
}
#[derive(Clone)]
pub struct Oracle11gSqlExecutor;
// 全局单例
pub const ORACLE11G_SQL_EXECUTOR: Oracle11gSqlExecutor = Oracle11gSqlExecutor;

pub struct OracleRow(oracle::Row);

impl RowType for OracleRow {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let values = self.0.sql_values();
        let value = &values[col_index];
        if value
            .is_null()
            .map_err(|e| DatabaseError::ConvertError(e.to_string()))?
        {
            return Ok(ParamValue::Null);
        }
        value_to_param_value(value).map_err(|e| DatabaseError::ConvertError(e.to_string()))
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let values = self.0.sql_values();
        let idx = self
            .0
            .column_info()
            .iter()
            .position(|info| info.name().eq_ignore_ascii_case(col_name));
        match idx {
            Some(i) => {
                let value = &values[i];
                if value
                    .is_null()
                    .map_err(|e| DatabaseError::ConvertError(e.to_string()))?
                {
                    return Ok(ParamValue::Null);
                }
                value_to_param_value(value)
                    .map_err(|e| DatabaseError::ConvertError(e.to_string()))
            }
            None => Err(DatabaseError::ConvertError(format!(
                "column not found: {}",
                col_name
            ))),
        }
    }
}
// 查询基本实现
impl Executor for Oracle11gSqlExecutor {
    type Row<'a> = OracleRow;
    type Conn = PooledConnection<OracleConnectionManager>;


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

            let rows = conn.query(sql.as_str(), &*to_sql_values).map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute query: {:?}", e)))?;
            let mut results = Vec::new();
            for row_result in rows {
                let row = row_result.map_err(|e| DatabaseError::ExecuteError(format!("Failed to fetch row: {:?}", e)))?;
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
        let stmt = conn.execute(sql.as_str(), &*to_sql_values).map_err(|e| DatabaseError::ExecuteError(format!("Failed to execute statement: {:?}", e)))?;
        let affected = stmt.row_count().map_err(|e| DatabaseError::ExecuteError(format!("Failed to get row count: {:?}", e)))?;
        Ok(affected)
    }

    fn get_conn_ref(&self) -> Result<Arc<Mutex<Self::Conn>>, DatabaseError> {
        let c = ORACLE11G_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("ORACLE11G_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        let p:Arc<DbManager<Pool<OracleConnectionManager>>> = DbManager::get_instance(get_datasource_name().as_str())?;
        let conn: PooledConnection<OracleConnectionManager> = p.get_pool().get().map_err(|e| DatabaseError::ConnectCanNotGetError(format!("Failed to get database connection: {:?}", e)))?;
        Ok(conn)
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.execute("SET TRANSACTION READ WRITE;", &[] as &[&dyn ToSql]).map_err(|e| DatabaseError::ExecuteError(format!("Failed to set transaction: {:?}", e)))?;
        Ok(())
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.commit().map_err(|e| DatabaseError::ExecuteError(format!("Failed to commit transaction: {:?}", e)))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let conn = conn.lock().await;
        conn.rollback().map_err(|e| DatabaseError::ExecuteError(format!("Failed to rollback transaction: {:?}", e)))?;
        Ok(())
    }

    async fn transaction_basic_exec<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output=Result<T, DatabaseError>>
    {
        with_conn_scope!(ORACLE11G_CONN_REGISTER, self, func)
    }

}

struct ParamValueWrapper(ParamValue);

impl ParamValueWrapper {

    /***
        Convert ParamValue -> ParamValueWrapper， 自己扩展用，以便数据兼容性的扩展
     */
    fn convert_param_values(param_values: &Vec<ParamValue>) -> Result<Vec<ParamValueWrapper>,DatabaseError> {
        param_values.iter().map(|param_value: &ParamValue|{
            Ok(ParamValueWrapper(param_value.clone()))
        }).collect()
    }

    /***
        ParamValue -> 数据库Value，查询组装参数用
     */
    fn as_sql_param(&self) -> Result<&dyn ToSql, DatabaseError> {
        match &self.0 {
            ParamValue::I64(v) => Ok(v),
            ParamValue::I32(v) => Ok(v),
            ParamValue::I16(v) => Ok(v),
            ParamValue::I8(v) => Ok(v),
            ParamValue::String(v) => Ok(v) ,
            ParamValue::F64(v) => Ok(v)      ,
            ParamValue::F32(v) => Ok(v),
            ParamValue::Bool(v) => Ok(v),
            ParamValue::Blob(v) => Ok(v),
            ParamValue::Clob(v) => Ok(v),
            ParamValue::U64(v) => Ok(v),
            ParamValue::U32(v) => Ok(v),
            ParamValue::U16(v) => Ok(v),
            ParamValue::U8(v) => Ok(v),
            ParamValue::DateTime(v) => Ok(v),
            _ => Err(DatabaseError::ConvertError(format!("Can't Convert Oracle Error: {:?}", self.0)))
        }
    }

}

fn value_to_param_value(value: &SqlValue) -> Result<ParamValue, oracle::Error> {
    // 1. 首先检查 NULL
    if value.is_null()? {
        return Ok(ParamValue::Null);
    }

    let oracle_type = value.oracle_type()?;
    match oracle_type {
        // 字符 / 大文本 / 对象 / 间隔类型 统一转字符串
        OracleType::Varchar2(_)
        | OracleType::NVarchar2(_)
        | OracleType::Char(_)
        | OracleType::NChar(_)
        | OracleType::Rowid
        | OracleType::CLOB
        | OracleType::NCLOB
        | OracleType::Long
        | OracleType::Json
        | OracleType::Xml
        | OracleType::Object(_)
        | OracleType::IntervalDS(_, _)
        | OracleType::IntervalYM(_)
        | OracleType::Float(_) => Ok(ParamValue::String(value.get::<String>()?)),

        // 二进制类型
        OracleType::Raw(_) | OracleType::BLOB | OracleType::BFILE | OracleType::LongRaw =>
            Ok(ParamValue::Blob(value.get::<Vec<u8>>()?)),

        // 二进制浮点
        OracleType::BinaryFloat => Ok(ParamValue::F32(value.get::<f32>()?)),
        OracleType::BinaryDouble => Ok(ParamValue::F64(value.get::<f64>()?)),

        // NUMBER：转 Decimal（避免精度丢失）
        OracleType::Number(_, _) => {
            let s = value.get::<String>()?;
            let dec = s
                .parse::<rust_decimal::Decimal>()
                .map_err(|e| {
                    oracle::Error::InvalidOperation(format!(
                        "failed to parse NUMBER '{}' as Decimal: {}",
                        s, e
                    ))
                })?;
            Ok(ParamValue::Decimal(dec))
        }

        // 日期 / 时间戳：转 DateTime<Local>
        OracleType::Date
        | OracleType::Timestamp(_)
        | OracleType::TimestampTZ(_)
        | OracleType::TimestampLTZ(_) => {
            let ts = value.get::<Timestamp>()?;
            Ok(ParamValue::DateTime(time_util::create_datetime_local(
                ts.year(),
                ts.month(),
                ts.day(),
                ts.hour(),
                ts.minute(),
                ts.second(),
                ts.nanosecond() / 1_000_000,
            )))
        }

        // 整数类型
        OracleType::Int64 => Ok(ParamValue::I64(value.get::<i64>()?)),
        OracleType::UInt64 => Ok(ParamValue::U64(value.get::<u64>()?)),

        // 不支持的类型
        OracleType::RefCursor | OracleType::Boolean => Err(oracle::Error::InvalidOperation(
            format!("unsupported Oracle type: {}", value.oracle_type()?),
        )),
    }
}

impl FromSql for ParamValueWrapper{
    fn from_sql(value: &SqlValue) -> oracle::Result<ParamValueWrapper> {
        Ok(ParamValueWrapper(value_to_param_value(&value)?))
    }
}