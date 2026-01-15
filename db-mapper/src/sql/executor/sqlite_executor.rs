use r2d2::PooledConnection;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Error, ToSql};
use std::cell::RefCell;
use std::collections::HashMap;
use rusqlite::ffi::sqlite3;
use rusqlite::types::{Type, ValueRef};
use crate::sql::pool::datasource::DbManager;

type SqliteSqlExecutor = SqlExecutor<SqliteConnectionManager>;

impl Executor for SqliteSqlExecutor{
    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn exec<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError> where E:Entity {
        println!("Executing: {}",sql);
        let db_manager = DbManager::get_instance();
        let conn:PooledConnection<SqliteConnectionManager> = db_manager.get_conn(None,None);
        let mut stmt = conn.prepare(sql)?;
        let param_vec:Vec<&dyn ToSql> = params.as_slice().iter().map(|x| to_sql(x)).collect::<Vec<_>>();
        let p_slien = param_vec.as_slice();
        let t_iter = stmt.query_map(p_slien, |row| {
            let mut e = E::new();
            for col in E::column_names(){
                let val = row.get_ref(col)?;
                let mut param_value=ParamValue::Null;
                match val {
                    ValueRef::Null => {  },
                    ValueRef::Integer(v) => { param_value = ParamValue::I64(v) },
                    ValueRef::Real(v) => {  param_value = ParamValue::F64(v) },
                    ValueRef::Text(v) => { 
                        let s = String::from_utf8(v.to_vec());
                        match s { 
                            Ok(s) => { param_value = ParamValue::String(s) },
                            Err(e) => { return Err(Error::FromSqlConversionFailure(0,Type::Text,Box::new(e))); },
                        }
                    },
                    ValueRef::Blob(v) => { param_value = ParamValue::Blob(v.to_vec()) },
                }
                e.set_value_by_column_name(col,param_value);
            }
            Ok(e)
        })?;
        let mut vec = Vec::new();
        for t in t_iter{
            vec.push(t?);
        }
        Ok(vec)
    }
}

pub fn to_sql(param_value: &ParamValue)->&dyn ToSql{
    match param_value{
        ParamValue::U64(x)=>{x as &dyn ToSql}
        ParamValue::U32(x)=>{x as &dyn ToSql}
        ParamValue::U16(x)=>{x as &dyn ToSql}
        ParamValue::U8(x)=>{x as &dyn ToSql}
        ParamValue::I64(x)=>{x as &dyn ToSql}
        ParamValue::I32(x)=>{x as &dyn ToSql}
        ParamValue::I16(x)=>{x as &dyn ToSql}
        ParamValue::I8(x)=>{x as &dyn ToSql}
        ParamValue::String(x)=>{x as &dyn ToSql}
        _ => {&0 as &dyn ToSql}
    }
}

