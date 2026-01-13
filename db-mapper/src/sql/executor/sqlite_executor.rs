use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::ToSql;
use crate::base::entity::Entity;

type SqliteSqlExecutor = SqlExecutor<SqliteConnectionManager>;

impl Executor for SqliteSqlExecutor{
    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn exec<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError> where E:Entity {
        println!("Executing: {}",sql);
        let conn = self.pool.get()?;
        let mut stmt = conn.prepare(sql)?;
        let param_vec:Vec<&dyn ToSql> = params.as_slice().iter().map(|x| to_sql(x)).collect::<Vec<_>>();
        let p_slien = param_vec.as_slice();
        let t_iter = stmt.query_map(p_slien, |row| {
            let mut e = E::new();
            // for col in E::column_names(){
            //     e.set_value_by_column_name(col,row.get(col)?);
            // }
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

