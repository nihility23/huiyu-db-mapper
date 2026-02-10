// use crate::base::entity::Entity;
// use crate::base::error::DatabaseError;
// use crate::base::error::DatabaseError::CommonError;
// use crate::base::param::ParamValue;
// use crate::sql::executor::Executor;
// use r2d2_sqlite::SqliteConnectionManager;
// use rusqlite::types::{Type, ValueRef};
// use rusqlite::{params, Error, Row, ToSql, Transaction};
// use std::cell::RefCell;
// use std::sync::OnceLock;
// use tokio::task_local;
//
// task_local! {
//     static TX: RefCell<Option<rusqlite::Transaction<'static>>>;
// }
//
// #[derive(Clone)]
// pub struct SqliteSqlExecutor<'a>;
// // 全局单例
// static SQLITE_SQL_EXECUTOR_CONFIG: OnceLock<SqliteSqlExecutor> = OnceLock::new();
// macro_rules! exec_basic_tx {
//     {
//         tx: $tx:expr,
//         sql: $sql:expr,
//         params: $params:expr,
//         map: $mapper:expr,
//         process: $processor:expr
//     } => {{
//             let mut stmt = $tx.prepare($sql)?;
//
//             let param_refs: Vec<&dyn ToSql> = $params
//                 .as_slice()
//                 .iter()
//                 .map(|x| to_sql(x))
//                 .collect();
//
//             let rows = stmt.query_map(&*param_refs, $mapper)?;
//             let mut results = Vec::new();
//
//             for row in rows {
//                 results.push(row?);
//             }
//
//             $processor(results)
//     }};
// }
//
// macro_rules! exec_basic {
//     {
//         sql: $sql:expr,
//         params: $params:expr,
//         map: $mapper:expr,
//         process: $processor:expr
//     } => {{
//         async {
//             let sql_c = $sql.to_string();
//             let params_c = $params.clone();
//
//             blocking::unblock(move || {
//
//                 let mut stmt = $tx.prepare(&sql_c)?;
//
//                 let param_refs: Vec<&dyn ToSql> = params_c
//                     .as_slice()
//                     .iter()
//                     .map(|x| to_sql(x))
//                     .collect();
//
//                 let rows = stmt.query_map(&*param_refs, $mapper)?;
//                 let mut results = Vec::new();
//
//                 for row in rows {
//                     results.push(row?);
//                 }
//
//                 $processor(results)
//             })
//             .await
//         }.await
//     }};
// }
//
// impl<'a> Executor<'a> for SqliteSqlExecutor<'a> {
//     type T = Transaction<'a>;
//
//     // fn get_sql_executor() -> &'static Self {
//     //     SQLITE_SQL_EXECUTOR_CONFIG.get_or_init(|| SqliteSqlExecutor)
//     // }
//
//     async fn query_some<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
//     where
//         E: Entity
//     {
//         // exec_basic!(sql: sql, params: params, map: make_e::<E>(), process: |results: Vec<E>| { Ok(results) })
//         todo!()
//     }
//
//     async fn query_one<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
//     where
//         E: Entity
//     {
//         // exec_basic!( tx: tx,sql: sql, params: params, map: make_e::<E>(), process: |results: Vec<E>| { Ok(results.into_iter().next()) })
//         todo!()
//     }
//
//
//     async fn insert(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>, DatabaseError>
//     {
//         // exec_basic!(tx: tx, sql: sql, params: params, map: |row|{
//         //     let id: ParamValue = value_to_param_value(row.get_ref(0).unwrap()).unwrap();
//         //     Ok(id)
//         // }, process: |results: Vec<ParamValue>| {
//         //     if results.is_empty() {
//         //         return Ok(None);
//         //     }
//         //     Ok(Some(results.into_iter().next().unwrap()))
//         // })
//         todo!()
//     }
//
//     async fn start_transaction(&self, tx: &Self::T) -> Result<(), DatabaseError> {
//         tx.execute("BEGIN", params![])?;
//         Ok(())
//     }
//
//     async fn commit(&self, tx: &Self::T) -> Result<(), DatabaseError> {
//         tx.execute("COMMIT", params![])?;
//         Ok(())
//     }
//
//     async fn rollback(&self, tx: &Self::T) -> Result<(), DatabaseError> {
//         tx.execute("ROLLBACK", params![])?;
//         Ok(())
//     }
//
//     async fn exec_tx(&self, tx: &Self::T) -> Result<(), DatabaseError> {
//         todo!()
//     }
//
//     fn get_sql_executor() -> &'a Self {
//         todo!()
//     }
// }
//
// fn value_to_param_value(value: ValueRef<'_>) -> Result<ParamValue, Error> {
//     let param_value;
//     match value {
//         ValueRef::Null => { param_value = ParamValue::Null },
//         ValueRef::Integer(v) => { param_value = ParamValue::I64(v) },
//         ValueRef::Real(v) => {  param_value = ParamValue::F64(v) },
//         ValueRef::Text(v) => {
//             let s = String::from_utf8(v.to_vec());
//             match s {
//                 Ok(s) => { param_value = ParamValue::String(s) },
//                 Err(e) => { return Err(Error::FromSqlConversionFailure(0,Type::Text,Box::new(CommonError("字符串转换错误".to_string())))); },
//             }
//         },
//         ValueRef::Blob(v) => { param_value = ParamValue::Blob(v.to_vec()) },
//     }
//     Ok(param_value)
// }
//
// const fn make_e<E>()->impl FnMut(&Row<'_>) -> rusqlite::Result<E> where E:Entity {
//     |row|{
//         let mut e = E::new();
//         for col in E::column_names(){
//             let val = row.get_ref(col)?;
//             let param_value = value_to_param_value(val)?;
//             e.set_value_by_column_name(col,param_value);
//         }
//         Ok(e)
//     }
// }
//
// pub fn to_sql(param_value: &ParamValue)->&dyn ToSql{
//     match param_value{
//         ParamValue::U64(x)=>{x as &dyn ToSql}
//         ParamValue::U32(x)=>{x as &dyn ToSql}
//         ParamValue::U16(x)=>{x as &dyn ToSql}
//         ParamValue::U8(x)=>{x as &dyn ToSql}
//         ParamValue::USize(x)=>{x as &dyn ToSql}
//         ParamValue::I64(x)=>{x as &dyn ToSql}
//         ParamValue::I32(x)=>{x as &dyn ToSql}
//         ParamValue::I16(x)=>{x as &dyn ToSql}
//         ParamValue::I8(x)=>{x as &dyn ToSql}
//         ParamValue::String(x)=>{x as &dyn ToSql}
//         ParamValue::F32(x)=>{x as &dyn ToSql}
//         ParamValue::F64(x)=>{x as &dyn ToSql}
//         ParamValue::Bool(x)=>{x as &dyn ToSql}
//         ParamValue::Blob(x)=>{x as &dyn ToSql}
//         ParamValue::Clob(x)=>{x as &dyn ToSql}
//         ParamValue::Null=>{&rusqlite::types::Null as &dyn ToSql}
//         ParamValue::DateTime(_)=>{&rusqlite::types::Null as &dyn ToSql}
//     }
// }
//
//
