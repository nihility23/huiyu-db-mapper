// use crate::base::db_type::DbType;
// use crate::base::entity::Entity;
// use crate::base::error::DatabaseError;
// use crate::base::page::{Page, PageRes};
// use crate::pool::datasource::get_datasource_type;
// use crate::pool::db_manager::DbManager;
// use crate::pool::transactional::get_transaction_id;
// use crate::query::query_wrapper::QueryWrapper;
// use r2d2::PooledConnection;
// use r2d2_mysql::MySqlConnectionManager;
// use r2d2_sqlite::SqliteConnectionManager;
// use crate::{exec_tx, exec_tx_with};
// use crate::sql::sql_generator::{BaseSqlGenerator, QueryWrapperSqlGenerator};
//
// pub trait BaseMapperTx<E, Tx>
// where
//     E: Entity,
// {
//     // select * from $table_name where $id = ?
//     async fn select_by_key(&self, tx: &Tx, key: &E::K) -> Result<Option<E>, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_select_by_key_sql::<E>(key.clone());
//         exec_tx_with!(tx, db_type, sql.as_str(), &vec![param_vec.clone()], query_one)
//     }
//
//     // select * from $table_name where $id in (?,...)
//     async fn select_by_keys(&self, tx: &Tx, keys: &Vec<E::K>) -> Result<Vec<E>, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_select_by_keys_sql::<E>(keys.clone());
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, query_some)
//     }
//
//     // delete from $table_name where $id = ?
//     async fn delete_by_key(&self, tx: &Tx, key: &E::K) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_delete_by_key_sql::<E>(&key);
//         exec_tx_with!(tx, db_type, sql.as_str(), &vec![param_vec.clone()], delete)
//     }
//
//     // delete from $table_name where $id in (?,...)
//     async fn delete_by_keys(&self, tx: &Tx, keys: &Vec<E::K>) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_delete_by_keys_sql::<E>(keys);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, delete)
//     }
//
//     // update $table_name set $column_name = ? where id = ?
//     async fn update_by_key(&self, tx: &Tx, e: &E) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_update_by_key_sql(e, false);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, update)
//     }
//
//     // insert $table_name into ($id,$column,...) values (?,?,...)
//     async fn insert(&self, tx: &Tx, entity: &E) -> Result<Option<E::K>, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_insert_one_sql::<E>(entity);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, E, insert)
//     }
//
//     // insert $table_name into ($id,$column,...) values (?,?,...),(?,?,...)
//     async fn insert_batch(&self, tx: &Tx, entities: &Vec<E>) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_insert_batch_sql::<E>(entities);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, E, insert_batch)
//     }
//
//     // select count(*) from (select * from $table_name where $column = ? ...)
//     // select * from $table_name where $column = ? ... limit ?,?
//     async fn select_page(
//         &self,
//         tx: &Tx,
//         page: Page,
//         query_wrapper: &QueryWrapper<'_, E>,
//     ) -> Result<PageRes<E>, DatabaseError> {
//         // let param_vec = vec![key.into()];
//         Ok(PageRes::new())
//     }
//
//     // select * from $table_name where $column = ? ...
//     async fn select(
//         &self,
//         tx: &Tx,
//         query_wrapper: &QueryWrapper<'_, E>,
//     ) -> Result<Vec<E>, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_query_sql::<E>(query_wrapper);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, query_some)
//     }
//
//     // select * from $table_name where $column = ? ... limit 1
//     async fn select_one(
//         &self,
//         tx: &Tx,
//         query_wrapper: &QueryWrapper<'_, E>,
//     ) -> Result<Option<E>, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_query_sql::<E>(query_wrapper);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, query_one)
//     }
//
//     // update $table_name set $column_name = ? where $column = ? ...
//     async fn update<'a>(
//         &self,
//         tx: &Tx,
//         entity: &E,
//         query_wrapper: &QueryWrapper<'a,E>,
//     ) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_update_sql::<E>(entity, query_wrapper, false);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, update)
//     }
//
//     async fn update_with_null<'a>(
//         &self,
//         tx: &Tx,
//         entity: &E,
//         query_wrapper: &QueryWrapper<'a, E>,
//     ) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_update_sql(entity, query_wrapper, true);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, update)
//     }
//
//     // delete from $table_name where $column = ? ...
//     async fn delete<'a>(&self, tx: &Tx, query_wrapper: &QueryWrapper<'a, E>) -> Result<u64, DatabaseError> {
//         let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//             "datasource type is null".to_string(),
//         ))?;
//         let (sql, param_vec) = db_type.gen_delete_sql(query_wrapper);
//         exec_tx_with!(tx, db_type, sql.as_str(), &param_vec, delete)
//     }
//
//
// }
