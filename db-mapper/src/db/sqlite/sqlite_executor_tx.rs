use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Mutex;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::error::DatabaseError::CommonError;
use crate::base::param::ParamValue;
use crate::pool::db_manager::DbManager;
use crate::sql::executor::Executor;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql, Transaction, TransactionBehavior};
use std::sync::{Arc, OnceLock};
use tokio::task_local;
use crate::sql::executor_tx::ExecutorTx;

task_local! {
    pub static SQLITE_TX_REGISTER : Arc<Mutex<(PooledConnection<SqliteConnectionManager>, Transaction<'static>)>>;
    // pub static SQLITE_CONN_REGISTER : Arc<Mutex<PooledConnection<SqliteConnectionManager>>>;
}
#[derive(Clone)]
pub struct SqliteSqlExecutorTx<'a>{
    _e: PhantomData<&'a ()>,
}
// 全局单例
pub const SQLITE_SQL_EXECUTOR_TX: SqliteSqlExecutorTx = SqliteSqlExecutorTx { _e: PhantomData };

// 查询基本实现
fn query_basic_tx<F,Q,M,T>(tx: &Transaction<'_>, sql: &str, params: &Vec<ParamValue>, f:F,q:Q) -> Result<T, DatabaseError> where F:FnMut(&Row<'_>) -> rusqlite::Result<M>, Q: Fn(Vec<M>) -> Result<T, DatabaseError> {
    let mut stmt = tx.prepare(sql)?;

    let param_refs: Vec<&dyn ToSql> = params
        .as_slice()
        .iter()
        .map(|x| to_sql(x))
        .collect();

    let rows = stmt.query_map(&*param_refs, f)?;
    let mut results = Vec::new();

    for row in rows {
        results.push(row?);
    }

    q(results)
   
}

// 执行基本实现
fn exec_basic_tx(tx: &Transaction<'_>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {

    let param_refs: Vec<&dyn ToSql> = params
        .as_slice()
        .iter()
        .map(|x| to_sql(x))
        .collect();
    let res = tx.execute(sql,&*param_refs)?;
    Ok(res as u64)
   
}

impl <'a>ExecutorTx for SqliteSqlExecutorTx<'a> {

    type Tx = Transaction<'a>;
    fn query_some_tx<E>(&self, tx: & Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        query_basic_tx(tx, sql, params, make_e::<E>(), |results: Vec<E>| { Ok(results) })
    }


    fn query_one_tx<E>(&self, tx: &Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        query_basic_tx(tx, sql, params, make_e::<E>(), |results: Vec<E>| { Ok(results.into_iter().next()) })
    }

    fn query_count_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        query_basic_tx(tx, sql, params, |row| { Ok(row.get(0)?) }, |results: Vec<u64>| { Ok(results.into_iter().sum()) })
    }

    fn insert_tx<E>(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError> where E:Entity
    {
        query_basic_tx(tx, sql, params, |row| {
                let val = row.get_ref(0)?;
                value_to_param_value(val)
            }, |results: Vec<ParamValue>| {
                    if results.is_empty() { Ok(None) } else { Ok(Some(results[0].clone().into()))
                }
            })
    }

    fn insert_batch_tx<E>(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        exec_basic_tx(tx, sql,  params)
    }

    fn delete_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic_tx(tx, sql, params)
    }

    fn update_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic_tx(tx, sql, params)
    }
}

fn value_to_param_value(value: ValueRef<'_>) -> Result<ParamValue, Error> {
    let param_value;
    match value {
        ValueRef::Null => { param_value = ParamValue::Null },
        ValueRef::Integer(v) => { param_value = ParamValue::I64(v) },
        ValueRef::Real(v) => {  param_value = ParamValue::F64(v) },
        ValueRef::Text(v) => {
            let s = String::from_utf8(v.to_vec());
            match s {
                Ok(s) => { param_value = ParamValue::String(s) },
                Err(e) => { return Err(Error::FromSqlConversionFailure(0,Type::Text,Box::new(CommonError("字符串转换错误".to_string())))); },
            }
        },
        ValueRef::Blob(v) => { param_value = ParamValue::Blob(v.to_vec()) },
    }
    Ok(param_value)
}

const fn make_e<E>()->impl FnMut(&Row<'_>) -> rusqlite::Result<E> where E:Entity {
    |row|{
        let mut e = E::new();
        for col in E::column_names(){
            let val = row.get_ref(col)?;
            let param_value = value_to_param_value(val)?;
            e.set_value_by_column_name(col,param_value);
        }
        Ok(e)
    }
}

pub fn to_sql(param_value: &ParamValue)->&dyn ToSql{
    match param_value{
        ParamValue::U64(x)=>{x as &dyn ToSql}
        ParamValue::U32(x)=>{x as &dyn ToSql}
        ParamValue::U16(x)=>{x as &dyn ToSql}
        ParamValue::U8(x)=>{x as &dyn ToSql}
        ParamValue::USize(x)=>{x as &dyn ToSql}
        ParamValue::I64(x)=>{x as &dyn ToSql}
        ParamValue::I32(x)=>{x as &dyn ToSql}
        ParamValue::I16(x)=>{x as &dyn ToSql}
        ParamValue::I8(x)=>{x as &dyn ToSql}
        ParamValue::String(x)=>{x as &dyn ToSql}
        ParamValue::F32(x)=>{x as &dyn ToSql}
        ParamValue::F64(x)=>{x as &dyn ToSql}
        ParamValue::Bool(x)=>{x as &dyn ToSql}
        ParamValue::Blob(x)=>{x as &dyn ToSql}
        ParamValue::Clob(x)=>{x as &dyn ToSql}
        ParamValue::Null=>{&rusqlite::types::Null as &dyn ToSql}
        ParamValue::DateTime(_)=>{&rusqlite::types::Null as &dyn ToSql}
    }
}


