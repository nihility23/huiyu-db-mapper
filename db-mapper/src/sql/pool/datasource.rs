use r2d2::{ManageConnection, Pool, PooledConnection};
use std::collections::HashMap;
use rusqlite::config::DbConfig;

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

    pub(crate) fn init(&self, db_configs: Vec<DbConfig>){

    }
}