use std::ffi::c_void;
use futures_util::future::TryFutureExt;
use chrono::{Datelike, Local, TimeZone, Timelike};
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::datasource::get_datasource_name;
use huiyu_db_mapper_core::pool::db_manager::DbManager;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::util::time_util;
use tracing::info;
use tokio::sync::Mutex;
use std::sync::Arc;
use chrono::LocalResult::Single;
use dameng_rust_sdk::{Connection, DamengClient, DamengValue};
use dameng_rust_sdk::prelude::handles::{CData, HasDataType};
use dameng_rust_sdk::prelude::{DataType, IntoParameter};
use dameng_rust_sdk::prelude::parameter::{CElement, InputParameter};
use dameng_rust_sdk::prelude::sys::CDataType;
use tokio::task_local;
use huiyu_db_mapper_core::base::param::ParamValue::Decimal;
use huiyu_db_mapper_core::with_conn_scope;

task_local! {
    pub static DAMENG_CONN_REGISTER : Arc<Mutex<Connection>>;
}
#[derive(Clone)]
pub struct DamengSqlExecutor;
// 全局单例
pub const DAMENG_SQL_EXECUTOR: DamengSqlExecutor = DamengSqlExecutor;

pub struct DamengRow{
    row: Vec<DamengValue>,
    column_names: Vec<String>,
}

impl RowType for DamengRow {
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
        let col_index = self.column_names.iter().position(|name| name == col_name);
        if col_index.is_none(){
            return Ok(ParamValue::Null);
        }
        let v = self.row.get(col_index.unwrap());
        if v.is_none(){
            return Ok(ParamValue::Null);
        }
        Ok(value_to_param_value(v.unwrap())?)
    }
}

impl Executor for DamengSqlExecutor {

    type Row<'a> = DamengRow;
    type Conn = Connection;

    async fn query<T, R, F, Q>(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static + Sync,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        let sql = sql.to_string();
        let params = params.clone();
        let mut conn = conn.lock().await;
        let mut vec = Vec::new();
        for param in params.iter() {
            vec.push(param_value_to_value(param)?);
        }
        let mut res = conn.query_with_param(&sql,vec.as_slice()).map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        let rows = res.fetch_all().map_err(|e| DatabaseError::RowConvertError(e.to_string()))?;
        let res = rows.iter().map(|row | {
            let res = mapper(&DamengRow{row: row.clone(), column_names: res.column_names().unwrap().to_vec()});
            res
        });
        let mut vec = Vec::new();
        for row in res {
            let row = row.map_err(|e| DatabaseError::RowConvertError(e.to_string()));
            vec.push(row?);
        }
        processor(vec)
    }

    async fn execute(&self, conn: Arc<Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let mut vec = Vec::new();
        for param in params.iter() {
            vec.push(param_value_to_value(param)?);
        }
        let sql = sql.to_string();
        let mut conn = conn.lock().await;
        let mut vec = Vec::new();
        for param in params.iter() {
            vec.push(param_value_to_value(param)?);
        }
        let res = conn.execute_with_param(&sql, vec.as_slice()).map_err(|e| DatabaseError::ConvertError(e.to_string()))?;
        if res {
            return Ok(1);
        }
        Ok(0)
    }


    fn get_conn_ref(&self)-> Result<Arc<Mutex<Self::Conn>>,DatabaseError> {
        let c = DAMENG_CONN_REGISTER.try_get();
        if c.is_err() {
            return Err(DatabaseError::AccessError("DAMENG_CONN_REGISTER is not set".to_string()));
        }
        Ok(c.unwrap())
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        let db_name = get_datasource_name();
        info!("get_conn: {}", db_name);
        let db_manager = DbManager::<DamengClient>::get_instance(db_name.as_str()).unwrap();
        let pool = db_manager.get_pool();
        let conn = pool.connect().map_err(|e| DatabaseError::ConnectCanNotGetError(e.to_string()))?;
        Ok(conn)
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let mut conn = conn.lock().await;
        conn.query("BEGIN").map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let mut conn = conn.lock().await;
        conn.query("COMMIT").map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        let conn = self.get_conn_ref()?;
        let mut conn = conn.lock().await;
        conn.query("ROLLBACK").map_err(|e| DatabaseError::ExecuteError(e.to_string()))?;
        Ok(())
    }

    async fn transaction_basic_exec<F, T, Fut>(&self, func: F) -> Result<T, DatabaseError>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output=Result<T, DatabaseError>>
    {
        with_conn_scope!(DAMENG_CONN_REGISTER, self, func)
    }

}

fn param_value_to_value(param_value: &ParamValue) -> Result<Box<dyn InputParameter>, DatabaseError> {
    match param_value {
        ParamValue::I8(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::I16(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::I32(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::I64(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::U8(v) => Ok(Box::new((*v as i64).into_parameter())),
        ParamValue::U16(v) => Ok(Box::new((*v as i64).into_parameter())),
        ParamValue::U32(v) => Ok(Box::new((*v as i64).into_parameter())),
        ParamValue::U64(v) => Ok(Box::new((*v as i64).into_parameter())),
        ParamValue::F32(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::F64(v) => Ok(Box::new(v.into_parameter())),
        ParamValue::String(v) => Ok(Box::new(v.clone().into_parameter())),
        ParamValue::Blob(v) => Ok(Box::new(v.clone().into_parameter())),
        ParamValue::Clob(v) => Ok(Box::new(v.clone().into_parameter())),
        ParamValue::DateTime(v) => Ok(Box::new(time_util::format_date_time_local(v,"%Y-%m-%d %H:%M:%S").into_parameter())),
        ParamValue::Null => Ok(Box::new(None::<String>.into_parameter())),
        ParamValue::Bool(v) => Ok(Box::new(if *v { 1 } else { 0 }) as Box<dyn InputParameter>),
        ParamValue::Decimal(v) => Ok(Box::new(v.as_f64().into_parameter())),
    }
}

fn value_to_param_value(value: &DamengValue) -> Result<ParamValue, DatabaseError> {
    match value {
        DamengValue::Int(v) => Ok(ParamValue::I32(  *v)),
        DamengValue::Decimal(v1,v2,v3) => Ok(ParamValue::Null),
        DamengValue::Float(v) => Ok(ParamValue::F64(*v)),
        DamengValue::Bool(v) => Ok(ParamValue::Bool(*v)),
        DamengValue::Null => Ok(ParamValue::Null),
        DamengValue::DateTime(v) => {
            let local_date_time = Local.from_local_datetime(&v);
            match local_date_time {
                Single(v) => Ok(ParamValue::DateTime(v.into())),
                _ => Err(DatabaseError::ConvertError("Invalid datetime".to_string())),
            }
        }
        DamengValue::Date(v) => {
            let local_date = Local.from_local_datetime(&v.clone().into());
            match local_date {
                Single(v) => Ok(ParamValue::DateTime(v.into())),
                _ => Err(DatabaseError::ConvertError("Invalid date".to_string())),
            }
        },
        DamengValue::BigInt(v) => Ok(ParamValue::I64(*v)),
        DamengValue::Binary(v) => Ok(ParamValue::Blob(v.clone())),
        DamengValue::String(v) => Ok(ParamValue::String(v.clone())),
    }
}
