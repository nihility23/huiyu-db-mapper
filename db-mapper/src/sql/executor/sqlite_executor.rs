use std::sync::OnceLock;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError::BusinessError;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor};
use crate::sql::pool::db_manager::DbManager;
use blocking::unblock;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql};

#[derive(Clone)]
pub struct SqliteSqlExecutor;
// 全局单例
static SQLITE_SQL_EXECUTOR_CONFIG: OnceLock<SqliteSqlExecutor> = OnceLock::new();

const fn make_e<E>()->impl FnMut(&Row<'_>) -> rusqlite::Result<E> where E:Entity {
    |row|{
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
                        Err(e) => { return Err(Error::FromSqlConversionFailure(0,Type::Text,Box::new(BusinessError("字符串转换错误".to_string())))); },
                    }
                },
                ValueRef::Blob(v) => { param_value = ParamValue::Blob(v.to_vec()) },
            }
            e.set_value_by_column_name(col,param_value);
        }
        Ok(e)
    }
}

impl Executor for SqliteSqlExecutor{

    fn get_sql_executor() -> &'static Self {
        SQLITE_SQL_EXECUTOR_CONFIG.get_or_init(||SqliteSqlExecutor)
    }

    async fn query_some<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        println!("Executing: {}",sql);
        let db_manager = DbManager::get_instance(db_name.unwrap_or("default"));
        if db_manager.is_none(){
            return Err(BusinessError("Can't get datasource!!!!".to_string()));
        }
        let sql_c = sql.to_string();
        let params_c = params.clone();
        unblock(move ||{
            let conn:PooledConnection<SqliteConnectionManager> = db_manager.unwrap().get_conn()?;
            let mut stmt = conn.prepare(sql_c.as_str())?;
            let param_vec:Vec<&dyn ToSql> = params_c.as_slice().iter().map(|x| to_sql(x)).collect::<Vec<_>>();

            let p_slien = param_vec.as_slice();

            let t_iter = stmt.query_map(p_slien, |row| {
                make_e()(row)
            })?;
            let mut vec = Vec::new();
            for t in t_iter{
                vec.push(t?);
            }
            return Ok(vec);
        }).await
    }

    async fn query_one<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        println!("Executing: {}",sql);
        let db_manager = DbManager::get_instance(db_name.unwrap_or("default"));
        if db_manager.is_none(){
            return Err(BusinessError("Can't get datasource!!!!".to_string()));
        }
        let sql_c = sql.to_string();
        let params_c = params.clone();
        unblock(move ||{
            let conn:PooledConnection<SqliteConnectionManager> = db_manager.unwrap().get_conn()?;
            let mut stmt = conn.prepare(sql_c.as_str())?;
            let param_vec:Vec<&dyn ToSql> = params_c.as_slice().iter().map(|x| to_sql(x)).collect::<Vec<_>>();

            let p_slien = param_vec.as_slice();

            let t_iter = stmt.query_map(p_slien, |row| {
                make_e()(row)
            })?;
            for t in t_iter{
                let a =t?;
                return Ok(Some(a));
            }
            return Ok(None);
        }).await
    }

    async fn exec<E, T>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<T, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn query_count<E, T>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<T, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn insert<E, T>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<T, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn update<E, T>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<T, DatabaseError>
    where
        E: Entity
    {
        todo!()
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

