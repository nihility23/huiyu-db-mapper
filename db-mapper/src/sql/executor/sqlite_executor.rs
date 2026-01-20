use crate::base::entity::Entity;
use crate::base::error::DatabaseError::BusinessError;
use crate::base::error::{DatabaseError, RowError};
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};
use crate::sql::pool::db_manager::DbManager;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql, TransactionBehavior};

type SqliteSqlExecutor = SqlExecutor<SqliteConnectionManager>;



impl Executor for SqliteSqlExecutor{
    type R<'a> = Row<'a>;

    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn row_to_entity<E>(row: &Self::R<'_>) -> Result<E, RowError>
    where
        E: Entity
    {
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
                        Err(e) => { return Err(RowError::TypeConversionError("字符串转换错误".to_string())); },
                    }
                },
                ValueRef::Blob(v) => { param_value = ParamValue::Blob(v.to_vec()) },
            }
            e.set_value_by_column_name(col,param_value);
        }
        Ok(e)
    }

    fn exec<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError> where E:Entity {
        println!("Executing: {}",sql);
        let db_manager = DbManager::get_instance(db_name.unwrap_or("default"));
        if db_manager.is_none(){
            return Err(BusinessError("Can't get datasource!!!!".to_string()));
        }
        let mut conn:PooledConnection<SqliteConnectionManager> = db_manager.unwrap().get().get()?;
        let tx =conn.transaction_with_behavior(TransactionBehavior::Immediate).unwrap();
        let mut stmt = tx.prepare(sql)?;
        let param_vec:Vec<&dyn ToSql> = params.as_slice().iter().map(|x| to_sql(x)).collect::<Vec<_>>();
        let p_slien = param_vec.as_slice();
        let t_iter = stmt.query_map(p_slien, |row| {
            let e = Self::row_to_entity(row);
            Ok(e.unwrap())
            // if e.is_ok(){
            //     Ok(e.unwrap())
            // }
            // match e.err().unwrap() {
            //     RowError::NotFoundError(str)=>{
            //         Err(Error::SqliteFailure(Error::NulError()))
            //     }
            // }
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

