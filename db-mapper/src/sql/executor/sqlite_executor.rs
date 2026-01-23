use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::error::DatabaseError::CommonError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::Executor;
use crate::sql::pool::db_manager::DbManager;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::{Type, ValueRef};
use rusqlite::{Error, Row, ToSql};
use std::sync::OnceLock;

#[derive(Clone)]
pub struct SqliteSqlExecutor;
// 全局单例
static SQLITE_SQL_EXECUTOR_CONFIG: OnceLock<SqliteSqlExecutor> = OnceLock::new();
macro_rules! exec_basic {
    {
        sql: $sql:expr,
        params: $params:expr,
        map: $mapper:expr,
        process: $processor:expr
    } => {{
        async {
            let sql_c = $sql.to_string();
            let params_c = $params.clone();

            blocking::unblock(move || {
                let db_manager = DbManager::get_instance()
                    .ok_or_else(|| DatabaseError::CommonError("Can't get datasource!!!!".to_string()))?;

                let conn: PooledConnection<SqliteConnectionManager> = db_manager.get_conn()?;
                let mut stmt = conn.prepare(&sql_c)?;

                let param_refs: Vec<&dyn ToSql> = params_c
                    .as_slice()
                    .iter()
                    .map(|x| to_sql(x))
                    .collect();

                let rows = stmt.query_map(&*param_refs, $mapper)?;
                let mut results = Vec::new();

                for row in rows {
                    results.push(row?);
                }

                $processor(results)
            })
            .await
        }.await
    }};
}
impl Executor for SqliteSqlExecutor {
    fn get_sql_executor() -> &'static Self {
        SQLITE_SQL_EXECUTOR_CONFIG.get_or_init(|| SqliteSqlExecutor)
    }

    async fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        exec_basic!(sql: sql, params: params, map: make_e::<E>(), process: |results: Vec<E>| { Ok(results) })
    }

    async fn query_one<E>(&self,  sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        exec_basic!( sql: sql, params: params, map: make_e::<E>(), process: |results: Vec<E>| { Ok(results.into_iter().next()) })
    }


    async fn insert(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>, DatabaseError>
    {
        exec_basic!(sql: sql, params: params, map: |row|{
            let id: ParamValue = value_to_param_value(row.get_ref(0).unwrap()).unwrap();
            Ok(id)
        }, process: |results: Vec<ParamValue>| {
            if results.is_empty() {
                return Ok(None);
            }
            Ok(Some(results.into_iter().next().unwrap()))
        })
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


