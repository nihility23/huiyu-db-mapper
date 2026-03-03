use std::cell::RefCell;
use std::rc::Rc;
use db_macros::Entity;
use db_mapper::base::config::DbConfig;
use db_mapper::base::db_type;
use db_mapper::base::db_type::DbType;
use db_mapper::base::error::DatabaseError;
use db_mapper::db::sqlite::sqlite_executor::{to_sql, SQLITE_CONN_REGISTER};
use db_mapper::pool::datasource::get_datasource_type;
use db_mapper::pool::db_manager::DbManager;
use db_mapper::query::base_mapper::BaseMapper;
use db_mapper::query::query_wrapper::QueryWrapper;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{ToSql, TransactionBehavior};
use rustlog::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task::spawn_blocking;
use uuid::Uuid;
use db_mapper::db::sqlite::sqlite_executor_tx::SQLITE_SQL_EXECUTOR_TX;
use db_mapper::pool::transactional::TransactionType;
use db_mapper::query::base_mapper_tx::BaseMapperTx;
use db_mapper::sql::executor_tx::ExecutorTx;

#[derive(Clone, Debug,Entity,Serialize,Deserialize)]
#[table(name = "t_app")]
#[derive(Default)]
pub struct AppEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<String>,
    #[field(column = "app_name")]
    pub app_name: Option<String>,
    #[field(column = "app_key")]
    pub app_key: Option<String>,
    #[field(column = "app_secret")]
    pub app_secret: Option<String>,
    #[field(column = "create_time")]
    pub create_time: Option<i64>,
}

pub struct AppMapper;

impl BaseMapper<AppEntity> for AppMapper {}
impl BaseMapperTx<AppEntity> for AppMapper {}

#[cfg(test)]
mod tests{
    use super::*;
    use db_mapper::base::config::DbConfig;
    use db_mapper::base::db_type::DbType;
    use db_mapper::base::param::ParamValue;
    use db_mapper::pool::db_manager::DbManager;
    use db_mapper::query::query_wrapper::QueryWrapper;
    use r2d2_sqlite::SqliteConnectionManager;

    #[test]
    fn test(){
        let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
        DbManager::initialize(&vec![db_config], |db_config|{
            let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
            let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
            pool
        });

        let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
        let res = AppMapper::select_one(&query_wrapper);
    }
}

pub async fn test(){
    let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
    DbManager::initialize(&vec![db_config], |db_config|{
    let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
    pool
    });

    // query one
    // let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("3".to_string()));
    // let res = AppMapper::select_one(&query_wrapper).await;
    // if res.is_err(){
    //     error!("Error: {}", res.err().unwrap());
    // }else {
    //
    //     let value = res.unwrap();
    //     println!("select one {}", serde_json::to_string_pretty(&value).unwrap());
    // }
    //
    // // query list
    // let query_wrapper = QueryWrapper::new().like("app_name", ParamValue::String("f".to_string()));
    // let res = AppMapper::select(&query_wrapper).await;
    // if res.is_err(){
    //     error!("Error: {}", res.err().unwrap());
    // }else {
    //     let value = res.unwrap();
    //     println!("query list {}", serde_json::to_string_pretty(&value).unwrap());
    // }
    //
    // // select_by_key
    // let res = AppMapper::select_by_key(&"2".to_string()).await;
    // let value = res.unwrap();
    // println!("select_by_key {}", serde_json::to_string_pretty(&value).unwrap());

    let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        )).unwrap();
        match db_type {
            DbType::Sqlite=>{
                // let db_manager = DbManager::<SqliteConnectionManager>::get_instance().unwrap();
                // let mut conn: PooledConnection<SqliteConnectionManager> = db_manager.get_inner_conn().unwrap();
                let conn:PooledConnection<SqliteConnectionManager> = DbManager::get_conn().unwrap();
                let mut conn_rc = Arc::new(conn);
                SQLITE_CONN_REGISTER.scope(conn_rc.clone(), async {
                    conn_rc.execute_batch("BEGIN IMMEDIATE").unwrap();

                    AppMapper::update_by_key(&AppEntity{ id: Some("3".to_string()), app_secret: Some("11221222".to_string()), ..Default::default() }).await;
                    AppMapper::update_by_key(&AppEntity{ id: Some("13".to_string()), app_secret: Some("22222222".to_string()), ..Default::default() }).await;
                    let res = AppMapper::select_one(&QueryWrapper::new().eq("id", ParamValue::String("3".to_string()))).await;
                    // insert
                    let mut entity = AppEntity::new();
                    entity.id = Some("113".to_string());
                    entity.app_name = Some("test".to_string());
                    entity.app_key = Some("test".to_string());
                    entity.app_secret = Some("test".to_string());
                    entity.create_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
                    let res = AppMapper::insert(&mut entity).await;
                    if res.is_err(){
                        error!("rollback Error: {}", res.err().unwrap());
                        conn_rc.execute_batch("ROLLBACK").unwrap();
                    }else{
                        info!("commit transaction");
                        conn_rc.execute_batch("COMMIT").unwrap();
                    }

                }).await;
            }
            _=>{}
        }
    // update by key
    // let mut entity = AppEntity::new();
    // entity.app_secret = Some(uuid::Uuid::new_v4().to_string().replace("-", ""));
    // entity.id = Some("2".to_string());
    // let res = AppMapper::update_by_key(&entity).await;
    // println!("update by key {:?}", json!(res.unwrap()));
    //
    // // delete by key
    // let res = AppMapper::delete_by_key(&"2".to_string()).await;
    // println!("delete by key {:?}", json!(res.unwrap()));
    //
    // // delete by wrapper
    // let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
    // let res = AppMapper::delete(&query_wrapper).await;
    // println!("delete by wrapper {:?}", json!(res.unwrap()));



    // let mut conn: PooledConnection<SqliteConnectionManager> = DbManager::get_instance().unwrap().get_conn().unwrap();
    // let mut tx = conn.transaction().unwrap();
    // let tx = TransactionType::SQLite(tx);
    // let u = AppMapper::select_by_key_tx(&tx, &"3".to_string());
    // println!("select by key {:?}", json!(u.await.expect("Failed to get result")));
    // SQLITE_SQL_EXECUTOR_TX.query_some_tx::<AppEntity>(
    //     &mut tx,
    //     "INSERT INTO t_app (id, app_name, app_key, app_secret, create_time) VALUES (?1, ?2, ?3, ?4, ?5)",
    //     &vec![]
    // );
    // tx.commit().unwrap();

}