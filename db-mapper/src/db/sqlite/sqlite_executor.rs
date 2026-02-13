use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::error::DatabaseError::CommonError;
use crate::base::param::ParamValue;
use crate::sql::executor::Executor;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql, Transaction};
use std::marker::PhantomData;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct SqliteSqlExecutor<'a>{
    _t:PhantomData<&'a ()>,
}
// 全局单例
static SQLITE_SQL_EXECUTOR_CONFIG: OnceLock<SqliteSqlExecutor> = OnceLock::new();

// 查询基本实现
fn query_basic<F,Q,M,T>(tx: &Transaction<'_>, sql: &str, params: &Vec<ParamValue>, f:F,q:Q) -> Result<T, DatabaseError> where F:FnMut(&Row<'_>) -> rusqlite::Result<M>, Q: Fn(Vec<M>) -> Result<T, DatabaseError> {
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
fn exec_basic(tx:&Transaction<'_>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
    let param_refs: Vec<&dyn ToSql> = params
        .as_slice()
        .iter()
        .map(|x| to_sql(x))
        .collect();
    let res = tx.execute(sql,&*param_refs)?;
    Ok(res as u64)
}

impl<'a> Executor for SqliteSqlExecutor<'a> {
    type T = Transaction<'a>;

    fn get_sql_executor() -> &'a Self {
        SQLITE_SQL_EXECUTOR_CONFIG.get_or_init(|| SqliteSqlExecutor { _t: PhantomData })
    }

    fn query_some<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        query_basic(tx, sql, params, make_e::<E>(), |results: Vec<E>| { Ok(results) })
    }


    fn query_one<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        query_basic(tx, sql, params, make_e::<E>(), |results: Vec<E>| { Ok(results.into_iter().next()) })
    }

    fn query_count(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        query_basic(tx, sql, params, |row| { Ok(row.get(0)?) }, |results: Vec<u64>| { Ok(results.into_iter().sum()) })
    }

    fn insert<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError> where E:Entity
    {
        query_basic(tx, sql, params, |row| {
                let val = row.get_ref(0)?;
                value_to_param_value(val)
            }, |results: Vec<ParamValue>| {
                    if results.is_empty() { Ok(None) } else { Ok(Some(results[0].clone().into()))
                }
            })
    }

    fn insert_batch<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        exec_basic(tx, sql,  params)
    }

    fn delete(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic(tx, sql, params)
    }

    fn update(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        exec_basic(tx, sql, params)
    }

    // fn start_transaction(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     tx.execute("BEGIN", params![])?;
    //     Ok(())
    // }
    //
    // fn commit(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     tx.execute("COMMIT", params![])?;
    //     Ok(())
    // }
    //
    // fn rollback(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     tx.execute("ROLLBACK", params![])?;
    //     Ok(())
    // }
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


