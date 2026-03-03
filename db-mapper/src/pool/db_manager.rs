use crate::base::config::DbConfig;
use crate::base::db_type::DbType;
use crate::pool::datasource::{get_datasource_name, set_datasource_type};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, OnceLock};
use std::fmt;
use std::time::Duration;
use deadpool::managed::{Manager, Object, Pool};
use crate::base::error::DatabaseError;

/// 使用 OnceLock 存储注册表，初始化后不可变
static DB_REGISTRY: OnceLock<Arc<DatabaseRegistry>> = OnceLock::new();

/// 数据库注册表 - 初始化后完全不可变
struct DatabaseRegistry {
    /// 实例映射
    instances: HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>,
    /// 默认实例映射（按类型）
    defaults: HashMap<TypeId, Arc<dyn Any + Send + Sync>>,
}

impl DatabaseRegistry {
    fn new() -> Self {
        Self {
            instances: HashMap::new(),
            defaults: HashMap::new(),
        }
    }

    /// 插入实例（仅在构建时使用）
    fn insert<M: Manager + 'static>(
        &mut self,
        name: String,
        instance: Arc<DbManager<M>>
    ) {
        let type_id = TypeId::of::<M>();
        let key = (name.clone(), type_id);

        self.instances.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);

        if !self.defaults.contains_key(&type_id) {
            self.defaults.insert(type_id, instance as Arc<dyn Any + Send + Sync>);
        }
    }

    /// 获取实例
    fn get_instance<M: Manager + 'static>(
        &self,
        name: &str
    ) -> Option<Arc<DbManager<M>>> {
        let key = (name.to_string(), TypeId::of::<M>());
        self.instances
            .get(&key)
            .and_then(|any| any.clone().downcast::<DbManager<M>>().ok())
    }

    /// 获取默认实例
    fn get_default<M: Manager + 'static>(&self) -> Option<Arc<DbManager<M>>> {
        self.defaults
            .get(&TypeId::of::<M>())
            .and_then(|any| any.clone().downcast::<DbManager<M>>().ok())
    }
}

/// 数据库管理器
pub struct DbManager<M: deadpool::managed::Manager> {
    /// 连接池
    pool: Pool<M>,
    /// 实例标签
    name: String,
    /// 数据库类型
    db_type: DbType,
}

impl<M: Manager + 'static> DbManager<M> {
    // ==================== 初始化方法 ====================

    /// 初始化所有数据库实例（应在程序启动时调用一次）
    pub fn initialize<F>(configs: &Vec<DbConfig>, factory: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&DbConfig) -> Pool<M> + Sync + Send + 'static,
    {
        // 检查是否已经初始化
        if DB_REGISTRY.get().is_some() {
            return Err("Database registry already initialized".into());
        }

        let mut registry = DatabaseRegistry::new();

        for config in configs {
            // 设置数据源类型
            set_datasource_type(config.name.clone(), config.db_type);

            // 创建连接池
            let pool = factory(&config);

            // 创建实例
            let instance = Arc::new(Self {
                pool,
                name: config.name.clone(),
                db_type: config.db_type,
            });

            // 插入到注册表
            registry.insert(config.name.clone(), instance);
        }

        // 一次性设置注册表
        DB_REGISTRY.set(Arc::new(registry))
            .map_err(|_| "Failed to set database registry".to_string())?;

        Ok(())
    }


    /// 静态方法：获取指定名称数据库的连接
    pub async fn get_conn() -> Result<Object<M>, DatabaseError>
    where
            M: Manager,
    {

        Self::get_instance().ok_or_else(|| {
            DatabaseError::NotFoundError("Database instance not found".to_string())
        })?.get_inner_conn().await
    }

    // ==================== 实例获取方法 ====================

    /// 获取指定名称的实例
    pub fn get_instance() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;

        let name = get_datasource_name().unwrap_or_else(|| "default".to_string());


        registry.get_instance::<M>(&name)
    }

    /// 获取默认实例
    pub fn default() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;
        registry.get_default::<M>()
    }

    /// 检查是否已初始化
    pub fn is_initialized() -> bool {
        DB_REGISTRY.get().is_some()
    }

    // ==================== 原有的实例方法 ====================

    /// 获取连接池
    #[inline]
    pub fn get_pool(&self) -> Pool<M> {
        self.pool.clone()
    }

    /// 获取连接池的引用
    #[inline]
    pub fn pool(&self) -> &Pool<M> {
        &self.pool
    }

    /// 获取数据库类型
    #[inline]
    pub fn get_db_type(&self) -> DbType {
        self.db_type
    }

    /// 获取连接
    pub async fn get_inner_conn(&self) -> Result<Object<M>, DatabaseError> {
        let conn = self.pool.get().await;//map_err(|e| DatabaseError::from(e))
        conn.map_err(|e| DatabaseError::NotFoundError(format!("Failed to get database connection")))
    }

    /// 获取实例名称
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// ==================== trait 实现 ====================

impl<M: Manager> Clone for DbManager<M> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            name: self.name.clone(),
            db_type: self.db_type,
        }
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
}