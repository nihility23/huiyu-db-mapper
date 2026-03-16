use crate::base::db_type::DbType;
use lazy_static::lazy_static;
use std::cell::RefCell;
use dashmap::DashMap;
use tracing::warn;
use tokio::task_local;

task_local! {
    pub static DB_NAME_REGISTRY: RefCell<Option<String>>;
}

lazy_static! {
    static ref DB_TYPE_REGISTRY: DashMap<String, DbType> = DashMap::new();
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
    DB_TYPE_REGISTRY.insert(name, data_type);
}

pub fn get_datasource_type_by_name(name: &str) -> Option<DbType> {
    let data_type = DB_TYPE_REGISTRY.get(name);
    if let Some(data_type) = data_type {
        return Some(data_type.value().clone());
    }
    None
}

pub fn get_datasource_type() -> Option<DbType> {
    get_datasource_type_by_name(&get_datasource_name())
}
