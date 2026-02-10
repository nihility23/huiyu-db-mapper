use crate::base::config::DbConfig;
use crate::base::db_type::DbType;
use crate::pool::datasource::{get_datasource_id, set_datasource_type};
use r2d2::{Error, ManageConnection, Pool, PooledConnection};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};

type RegistryMap = HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>;
static DB_REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();



pub struct DbManager<T:ManageConnection>{
    /// 核心数据存储
    pool_data: Arc<RwLock<Pool<T>>>,

    /// 实例标签
    name: String,

    db_type: DbType
}

impl<T: ManageConnection> DbManager<T> {

    pub fn register_db<F>(db_config: &DbConfig, f: F) where F: FnOnce(&DbConfig) -> Pool<T> {
        set_datasource_type(db_config.name.clone(), db_config.db_type);
        // 类型别名，提高代码可读性
        let registry = DB_REGISTRY.get_or_init(|| RwLock::new(RegistryMap::new()));
        let key = (db_config.name.to_string(), TypeId::of::<T>());

        // 快速路径：读锁查找
        {
            let registry_guard = registry.read().unwrap();
            if let Some(existing) = registry_guard.get(&key) {
                if let Ok(_) = existing.clone().downcast::<Self>() {
                    return ;
                }
            }
        }

        // 慢路径：创建新实例
        let mut registry_guard = registry.write().unwrap();

        // 双重检查
        if let Some(existing) = registry_guard.get(&key) {
            if let Ok(_) = existing.clone().downcast::<Self>() {
                return;
            }
        }

        // 创建并注册
        let instance = Arc::new(Self {
            pool_data: Arc::new(RwLock::new(f(db_config))),
            name: db_config.name.to_string(),
            db_type: db_config.db_type,
        });
        if registry_guard.is_empty() {
            registry_guard.insert(("default".to_string(),TypeId::of::<T>()), instance.clone() as Arc<dyn Any + Send + Sync>);
        }
        registry_guard.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);
    }

    pub fn get_instance() -> Option<Arc<Self>>
    {
        let db_name_opt = get_datasource_id();
        let db_name;
        if db_name_opt.is_none() {
            db_name = "default".to_string();
        } else {
            db_name = db_name_opt.unwrap();
        }
        let registry = DB_REGISTRY.get_or_init(|| RwLock::new(RegistryMap::new()));
        let key = (db_name.to_string(), TypeId::of::<T>());

        // 快速路径：读锁查找
        {
            let registry_guard = registry.read().unwrap();
            if let Some(existing) = registry_guard.get(&key) {
                if let Ok(typed) = existing.clone().downcast::<Self>() {
                    return Some(typed);
                }
            }
        }

        // 慢路径：创建新实例
        let mut registry_guard = registry.write().unwrap();

        // 双重检查
        if let Some(existing) = registry_guard.get(&key) {
            if let Ok(typed) = existing.clone().downcast::<Self>() {
                return Some(typed);
            }
        }
        None
    }

    /// 获取数据（克隆版本）
    pub fn get_pool(&self) -> Pool<T>
    {
        self.pool_data.read().unwrap().clone()
    }
    pub fn get_db_type(&self) -> DbType{
        self.db_type
    }
    pub fn get_conn(&self) -> Result<PooledConnection<T>,Error>
    {
         self.get_pool().get()
    }
}
