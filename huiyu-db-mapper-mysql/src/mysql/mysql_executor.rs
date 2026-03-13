use chrono::{Datelike, Timelike};
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::util::time_util;
use mysql::prelude::{Queryable};
use mysql::Error::FromRowError;
use mysql::{Params, Pool, PooledConn, Row, Value};
use rustlog::info;
use std::sync::Mutex;
use std::sync::Arc;
use std::time;
use tokio::task::spawn_blocking;
use tokio::task_local;
use huiyu_db_mapper_core::base::entity::ColumnInfo;

task_local! {
    pub static MYSQL_CONN_REGISTER : Arc<Mutex<PooledConn>>;
}
#[derive(Clone)]
pub struct MysqlSqlExecutor;
// 全局单例
pub const MYSQL_SQL_EXECUTOR: MysqlSqlExecutor = MysqlSqlExecutor;

pub struct MysqlRow{
    row: Row
}

impl RowType for MysqlRow {
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.row.get(col_index);
        if v.is_none(){
            return Ok(ParamValue::Null);
        }
        Ok(value_to_param_value(v.unwrap())?)
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        let v = self.row.get(col_name);
        if v.is_none(){
            return Ok(ParamValue::Null);
        }
        Ok(value_to_param_value(v.unwrap())?)
    }
}

impl Executor for MysqlSqlExecutor {

    type Row<'a> = MysqlRow;
    type Conn = PooledConn;

    async fn query<T, R, F, Q>(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        let sql = sql.to_string();
        let params = params.clone();
        spawn_blocking(move || {
            let stat = conn.lock().unwrap().prep(sql).map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
            let mut vec = Vec::new();
            for param in params.iter() {
                vec.push(param_value_to_value(param)?);
            }
            let res = conn.lock().unwrap().exec_map(stat, Params::Positional(vec),|row: Row|{
                let res = mapper(&MysqlRow{row: row.clone()}).map_err(|_| FromRowError(row));
                res
            }).map_err(|e| DatabaseError::RowConvertError(e.to_string()))?;
            let mut vec = Vec::new();
            for row in res {
                let row = row.map_err(|e| DatabaseError::RowConvertError(e.to_string()));
                vec.push(row?);
            }
            processor(vec)
        }).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?
    }

    async fn execute(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let mut vec = Vec::new();
        for param in params.iter() {
            vec.push(param_value_to_value(param)?);
        }
        let res:Option<Value> = conn.lock().unwrap().exec_first(sql, Params::Positional(vec)).map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        if res.is_none() {
            return Ok(0);
        }
        Ok(value_to_param_value(res.unwrap())?.into())
    }


    fn get_conn_ref(&self)-> Result<Arc<Mutex<Self::Conn>>,DatabaseError> {
        let c = MYSQL_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("MYSQL_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        let db_name = get_datasource_name();
        spawn_blocking(move || {
            info!("get_conn: {}", db_name);
            let db_manager = DbManager::<Pool>::get_instance(db_name.as_str()).unwrap();
            let pool = db_manager.get_pool();
            pool.get_conn().map_err(|e| DatabaseError::ConnectCanNotGetError(e.to_string()))
        }).await.map_err(|e| DatabaseError::ConvertError(e.to_string()))?
    }
}

fn param_value_to_value(val: &ParamValue) -> Result<Value, DatabaseError> {
    match val {
        ParamValue::U64(x)=>Ok(Value::UInt(*x)),
        ParamValue::U8(x)=>Ok(Value::UInt(*x as u64)),
        ParamValue::U16(x)=>Ok(Value::UInt(*x as u64)),
        ParamValue::U32(x)=>Ok(Value::UInt(*x as u64)),
        ParamValue::I8(x)=>Ok(Value::Int(*x as i64)),
        ParamValue::I16(x)=>Ok(Value::Int(*x as i64)),
        ParamValue::I32(x)=>Ok(Value::Int(*x as i64)),
        ParamValue::I64(x)=>Ok(Value::Int(*x)),
        ParamValue::F32(x)=>{Ok(Value::Float(*x))},
        ParamValue::F64(x)=>{Ok(Value::Double(*x))}
        ParamValue::Blob(x)=>{Ok(Value::Bytes(x.to_vec()))}
        ParamValue::String(x)=>{Ok(Value::Bytes(x.to_string().into_bytes()))}
        ParamValue::DateTime(x)=>{
            Ok(Value::Date(
                x.year() as u16,
                x.month() as u8,
                x.day() as u8,
                x.hour() as u8,
                x.minute() as u8,
                x.second() as u8,
                x.nanosecond()/ 1000u32,
            ))
        }
        ParamValue::Null=>Ok(Value::NULL),
        _=>Err(DatabaseError::ConvertError("Unsupported parameter type".to_string()))
    }
}

fn value_to_param_value(value: Value) -> Result<ParamValue, DatabaseError> {
    match value {
        Value::Int(v) => Ok(ParamValue::I64(v)),
        Value::UInt(v) => Ok(ParamValue::U64(v)),
        Value::Float(v) => Ok(ParamValue::F32(v)),
        Value::Double(v) => Ok(ParamValue::F64(v)),
        Value::Bytes(v) => Ok(ParamValue::Blob(v)),
        Value::Date(year, month, day, hour, minutes, seconds, micro) => Ok(ParamValue::DateTime(time_util::create_datetime_local(
            year as i32, month as u32, day as u32, hour as u32, minutes as u32, seconds as u32, micro
        ))),
        Value::Time(is_negative, days, hours, minutes, seconds, micro_seconds) => {
            let duration = time::Duration::from_days(days as u64)
                +time::Duration::from_hours(hours as u64)
                +time::Duration::from_mins(minutes as u64)
                +time::Duration::from_secs(seconds as u64)
                +time::Duration::from_micros(micro_seconds as u64);
            if is_negative {
                return Ok(ParamValue::I64(0 - duration.as_micros() as i64));
            }
            Ok(ParamValue::I64(duration.as_micros() as i64))
        },
        Value::NULL => Ok(ParamValue::Null),
    }
}
