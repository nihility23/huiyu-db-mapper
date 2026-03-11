use crate::mapper::{AppMapper, BedMapper};
use db_macros::Entity;
use db_mapper::base::config::DbConfig;
use db_mapper::base::db_type;
use db_mapper::base::db_type::DbType;
use db_mapper::base::error::DatabaseError;
use db_mapper::base::param::ParamValue;
use db_mapper::db::sqlite::sqlite_executor::{to_sql, SQLITE_CONN_REGISTER};
use db_mapper::pool::datasource::{get_datasource_type, DB_NAME_REGISTRY};
use db_mapper::pool::db_manager::DbManager;
use db_mapper::query::base_mapper::BaseMapper;
use db_mapper::query::query_wrapper::QueryWrapper;
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{tokio_postgres, ManagerConfig, RecyclingMethod, Runtime};
use deadpool_sqlite::{Config, Manager, Object};
use rusqlite::fallible_iterator::FallibleIterator;
use rusqlite::{ToSql, TransactionBehavior};
use rustlog::{error, info};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use uuid::Uuid;

pub async fn test(){
    let db_config_sqlite = DbConfig::new(DbType::Sqlite, None, None,None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()),  None, "default".to_string());
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                  Some("10.150.2.200".to_string()),
                                  Some(5432),
                                  Some("postgres".to_string()),
                                  Some("123456".to_string()),
                                  Some("postgres".to_string()),
                                  Some("fhds".to_string()),
                                  "postgres".to_string());
    DbManager::register(&db_config_sqlite, |db_config| {
        Config::new(db_config.database.clone().expect("Database URL is missing")).create_pool(deadpool_sqlite::Runtime::Tokio1).expect("Failed to create pool")
    }).expect("TODO: panic message");
    //
    //query one
    let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("113".to_string()));
    let res = AppMapper::select_one(&query_wrapper).await;
    if res.is_err(){
        error!("Error: {}", res.err().unwrap());
    }else {

        let value = res.unwrap();
        println!("select one {}", serde_json::to_string_pretty(&value).unwrap());
    }

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
    // let res = AppMapper::select_by_key(&"113".to_string()).await;
    // let value = res.unwrap();
    // println!("select_by_key {}", serde_json::to_string_pretty(&value).unwrap());



    DbManager::register(&db_config_postgres, |db_config| {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.dbname = Some("postgres".to_string());
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg.user = Some("postgres".to_string());
        cfg.password = Some("123456".to_string());
        cfg.host = Some(String::from("10.150.2.200"));
        cfg.port = Some(5432);
        // // 或者
        cfg.options = Some("--search_path=fhds".to_string());
        let pool:deadpool_postgres::Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls ).unwrap();
        pool
    }).expect("TODO: panic message");



    DB_NAME_REGISTRY.scope(RefCell::new(Some("postgres".to_string())), async {

        let res = BedMapper::select_by_key(&"bed014".to_string()).await;
        if res.is_err(){
            error!("Error: {}", res.err().unwrap());
        }else {
            let value = res.unwrap();
            println!("select_by_key {}", serde_json::to_string_pretty(&value).unwrap());
        }


    }).await;

    // let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
    //         "datasource type is null".to_string(),
    //     )).unwrap();
    //     match db_type {
    //         DbType::Sqlite=>{
    //             // let db_manager = DbManager::<SqliteConnectionManager>::get_instance().unwrap();
    //             // let mut conn: PooledConnection<SqliteConnectionManager> = db_manager.get_inner_conn().unwrap();
    //             let conn:Object = DbManager::get_instance().await.unwrap();
    //             let mut conn_rc = Arc::new(Mutex::new(conn));
    //             SQLITE_CONN_REGISTER.scope(conn_rc.clone(), async {
    //                 conn_rc.lock().await.interact(|conn| conn.execute("BEGIN IMMEDIATE", [])).await.unwrap();
    //                 AppMapper::update_by_key(&AppEntity{ id: Some("3223".to_string()), app_secret: Some("AAAAAA".to_string()), ..Default::default() }).await;
    //                 AppMapper::update_by_key(&AppEntity{ id: Some("113".to_string()), app_secret: Some("DDDFDD".to_string()), ..Default::default() }).await;
    //                 let res = AppMapper::select_one(&QueryWrapper::new().eq("id", ParamValue::String("3".to_string()))).await;
    //                 // insert
    //                 let mut entity = AppEntity::new();
    //                 entity.id = Some("88c1f0c8843448e589fe0854f96b93a4".to_string());
    //                 entity.app_name = Some("test".to_string());
    //                 entity.app_key = Some("test".to_string());
    //                 entity.app_secret = Some("test".to_string());
    //                 entity.create_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    //                 let res = AppMapper::insert(&mut entity).await;
    //                 if res.is_err(){
    //                     error!("rollback Error: {}", res.err().unwrap());
    //                     conn_rc.lock().await.interact(|conn| conn.execute("ROLLBACK", [])).await.unwrap();
    //                 }else{
    //                     info!("commit transaction");
    //                     conn_rc.lock().await.interact(|conn| conn.execute("COMMIT", [])).await.unwrap();
    //                 }
    //
    //             }).await;
    //         }
    //         _=>{}
    //     }
    // // update by key
    // let mut entity = AppEntity::new();
    // entity.app_secret = Some(uuid::Uuid::new_v4().to_string().replace("-", ""));
    // entity.id = Some("13".to_string());
    // let res = AppMapper::update_by_key(&entity).await;
    // println!("update by key {:?}", json!(res.unwrap()));
    //
    // // delete by key
    // let res = AppMapper::delete_by_key(&"13".to_string()).await;
    // println!("delete by key {:?}", json!(res.unwrap()));
    //
    // // delete by wrapper
    // let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("3".to_string()));
    // let res = AppMapper::delete(&query_wrapper).await;
    // println!("delete by wrapper {:?}", json!(res.unwrap()));


}