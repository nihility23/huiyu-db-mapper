use crate::base::config::DbConfig;
use crate::base;
use r2d2::{Builder, ManageConnection, Pool, PooledConnection};
use r2d2_mysql::MySqlConnectionManager;
use r2d2_sqlite::SqliteConnectionManager;
use std::collections::HashMap;

pub(crate) struct DbManager<T: ManageConnection> {
    pool_map: HashMap<String,Pool<T>>,
    tx_map: HashMap<String,PooledConnection<T>>
}

// 连接是获取所有权
// pool获取副本
impl <T:ManageConnection> DbManager<T>{
    pub(crate) fn get_conn(&mut self, db_name: Option<String>, tx_id_opt : Option<String>)->PooledConnection<T>{
        if tx_id_opt.is_some(){
            let tx_id = tx_id_opt.unwrap();
            let v = self.tx_map.remove(&tx_id);
            if let Some(v) = v{
                return v;
            }
        }
        let p = self.pool_map.get(&db_name.unwrap());
        let conn = p.unwrap().get().unwrap();
        conn
    }

    pub(crate) fn store_tx_conn(&mut self, tx_id : Option<String>, conn: PooledConnection<T>){
        self.tx_map.insert(tx_id.unwrap(), conn);
    }

    pub(crate) fn release_conn(&mut self, db_name: Option<String>, conn: PooledConnection<T>){
        // drop自动归还
    }

    pub(crate) fn init(&self, db_configs: &Vec<DbConfig>){
        for db_config in db_configs {
            match db_config.db_type{
                base::db_type::DbType::Mysql=>{
                    let db_manager:&DbManager<MySqlConnectionManager> = DbManager::get_instance();
                    // db_manager.pool_map.insert(db_config.name.unwrap(),)
                }
                base::db_type::DbType::Oracle=>{
                    let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
                }
                base::db_type::DbType::Postgres=>{
                    let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
                }
                base::db_type::DbType::Sqlite=>{
                    let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
                }
                base::db_type::DbType::SqlServer=>{
                    let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
                }
                _=>{

                }
            }
        }
    }

    fn get_instance() -> &'static DbManager<T> {
        Box::leak(Box::new(DbManager { pool_map: HashMap::new(), tx_map: HashMap::new() }))
    }
}

fn test(){
    let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
    db_manager.init(&Vec::new());
    let db_manager:&DbManager<MySqlConnectionManager> = DbManager::get_instance();
    db_manager.init(&Vec::new());
}