use crate::base::db_type::DbType;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::RwLock;
use tracing::warn;
use tokio::task_local;

task_local! {
    pub static DB_NAME_REGISTRY: RefCell<Option<String>>;
}

lazy_static! {
    static ref DB_TYPE_REGISTRY: RwLock<HashMap<String, DbType>> = RwLock::new(HashMap::new());
}

pub fn get_datasource_name() -> String {
    if let Some(name) = DB_NAME_REGISTRY.try_get().ok() {
        let name = name.borrow().clone();
        if name.is_some() {
            return name.unwrap();
        }
    }
    "default".to_string()
}

pub(crate) fn set_datasource_type(name: String, data_type: DbType) {
    warn!("set_datasource_type: {} {}", name, data_type);
    DB_TYPE_REGISTRY.write().unwrap().insert(name, data_type);
}

pub fn get_datasource_type_by_name(name: &str) -> Option<DbType> {
    DB_TYPE_REGISTRY.read().unwrap().get(name).cloned()
}

pub fn get_datasource_type() -> Option<DbType> {
    get_datasource_type_by_name(&get_datasource_name())
}
