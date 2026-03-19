use crate::base::config::DbConfig;
use crate::base::db_type::DbType;
use crate::base::error::DatabaseError;
use crate::pool::datasource::{get_datasource_name, set_datasource_type};
use dashmap::DashMap;
use tracing::{info, trace, warn};
use std::any::{Any, TypeId};
use std::error::Error;
use std::sync::{Arc, OnceLock};

/// 使用 OnceLock 存储注册表，内部使用 DashMap 实现高性能并发访问
static DB_REGISTRY: OnceLock<Arc<DatabaseRegistry>> = OnceLock::new();

/// 数据库注册表 - 使用 DashMap 实现无锁并发
struct DatabaseRegistry {
    /// 实例映射：使用 DashMap 实现线程安全的并发访问
    instances: DashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>,
    /// 默认实例映射
    defaults: DashMap<TypeId, Arc<dyn Any + Send + Sync>>,
    // 实例元数据（可选，用于存储额外信息）
    // metadata: DashMap<String, InstanceMetadata>,
}

/// 实例元数据
// #[derive(Debug, Clone)]
// pub struct InstanceMetadata {
//     // db_type: DbType,
//     // created_at: std::time::SystemTime,
//     config: HashMap<String, String>,
// }

impl DatabaseRegistry {
    fn new() -> Self {
        Self {
            instances: DashMap::new(),
            defaults: DashMap::new(),
            // metadata: DashMap::new(),
        }
    }

    /// 插入实例
    fn insert<M: Send + Sync + 'static>(
        &self,
        name: String,
        instance: Arc<DbManager<M>>,
        // config: &DbConfig,
    ) {
        let type_id = TypeId::of::<M>();
        let key = (name.clone(), type_id);
        warn!("Inserting instance with key: {:?}", key);
        // 插入实例
        self.instances.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);

        // 存储元数据
        // let metadata = InstanceMetadata {
            // db_type: config.db_type,
            // created_at: std::time::SystemTime::now(),
            // config: HashMap::new(), // 可以根据需要填充配置信息
        // };
        // self.metadata.insert(name.clone(), metadata);

        // 设置默认实例（如果没有该类型的默认实例）
        self.defaults.entry(type_id).or_insert(instance as Arc<dyn Any + Send + Sync>);
    }

    /// 更新默认实例
    // fn set_default<M: Send + Sync + 'static>(
    //     &self,
    //     instance: Arc<DbManager<M>>
    // ) {
    //     let type_id = TypeId::of::<M>();
    //     self.defaults.insert(type_id, instance as Arc<dyn Any + Send + Sync>);
    // }

    /// 获取实例
    fn get_instance<M: Send + Sync + 'static>(
        &self,
        name: &str
    ) -> Option<Arc<DbManager<M>>> {
        let key = (name.to_string(), TypeId::of::<M>());
        trace!("Getting instance with key: {:?}", key);
        self.instances
            .get(&key)
            .and_then(|entry| entry.value().clone().downcast::<DbManager<M>>().ok())
    }

    /// 获取默认实例
    fn get_default<M: Send + Sync + 'static>(&self) -> Option<Arc<DbManager<M>>> {
        self.defaults
            .get(&TypeId::of::<M>())
            .and_then(|entry| entry.value().clone().downcast::<DbManager<M>>().ok())
    }

    /// 检查是否存在实例
    fn contains<M: Send + Sync + 'static>(&self, name: &str) -> bool {
        let key = (name.to_string(), TypeId::of::<M>());
        self.instances.contains_key(&key)
    }

    /// 移除实例
    fn remove<M: Send + Sync + 'static>(
        &self,
        name: &str
    ) -> Option<Arc<DbManager<M>>> {
        let key = (name.to_string(), TypeId::of::<M>());

        // 从 instances 中移除
        if let Some((_, instance)) = self.instances.remove(&key) {
            // 也从 metadata 中移除
            // self.metadata.remove(name);

            // 如果这是默认实例，需要处理默认实例的更新
            let type_id = TypeId::of::<M>();
            if let Some(default_entry) = self.defaults.get(&type_id) {
                if let Ok(default_instance) = default_entry.value().clone().downcast::<DbManager<M>>() {
                    if default_instance.get_name() == name {
                        // 如果移除的是默认实例，尝试设置新的默认实例
                        self.defaults.remove(&type_id);
                        // 尝试找另一个同类型的实例作为默认
                        if let Some(first_instance) = self.get_first_instance::<M>() {
                            self.defaults.insert(type_id, first_instance as Arc<dyn Any + Send + Sync>);
                        }
                    }
                }
            }

            instance.downcast::<DbManager<M>>().ok()
        } else {
            None
        }
    }

    /// 获取第一个同类型实例（用于默认实例失效后的替补）
    fn get_first_instance<M: Send + Sync + 'static>(&self) -> Option<Arc<DbManager<M>>> {
        let type_id = TypeId::of::<M>();
        self.instances
            .iter()
            .find(|entry| entry.key().1 == type_id)
            .and_then(|entry| entry.value().clone().downcast::<DbManager<M>>().ok())
    }

    /// 获取所有实例名称
    fn list_instances<M: Send + Sync + 'static>(&self) -> Vec<String> {
        let type_id = TypeId::of::<M>();
        self.instances
            .iter()
            .filter(|entry| entry.key().1 == type_id)
            .map(|entry| entry.key().0.clone())
            .collect()
    }

    /// 获取实例数量
    fn instance_count(&self) -> usize {
        self.instances.len()
    }

    // 获取实例元数据
    // fn get_metadata(&self, name: &str) -> Option<InstanceMetadata> {
    //     self.metadata.get(name).map(|entry| entry.clone())
    // }

    // 清空所有实例（谨慎使用）
    // fn clear(&self) {
    //     self.instances.clear();
    //     self.defaults.clear();
    //     // self.metadata.clear();
    // }
}

/// 数据库管理器
pub struct DbManager<M: Send + Sync + 'static> {
    /// 连接池
    pool: M,
    /// 实例标签
    name: String,
    /// 数据库类型
    db_type: DbType,
}

impl<M: Send + Sync + 'static> DbManager<M> {
    /// 初始化注册表（只需调用一次）
    pub fn init_registry() {
        DB_REGISTRY.get_or_init(|| Arc::new(DatabaseRegistry::new()));
    }

    /// 注册新的数据库实例（可以多次调用）
    pub fn register<F>(config: &DbConfig, factory: F) -> Result<Arc<Self>, DatabaseError>
    where
        F: Fn(&DbConfig) -> Result<M,DatabaseError> + Sync + Send + 'static,
    {
        // 确保注册表已初始化
        Self::init_registry();

        let registry = DB_REGISTRY.get().unwrap();

        // 检查是否已存在同名实例
        if registry.contains::<M>(&config.name) {
            return Err(DatabaseError::InstanceAlreadyExistsError(config.name.clone()));
        }

        // 设置数据源类型
        set_datasource_type(config.name.clone(), config.db_type);

        // 创建连接池
        let pool = factory(config)?;

        // 创建实例
        let instance = Arc::new(Self {
            pool,
            name: config.name.clone(),
            db_type: config.db_type,
        });

        // 插入到注册表
        registry.insert(config.name.clone(), instance.clone());
        // registry.insert(config.name.clone(), instance.clone(), config);


        Ok(instance)
    }

    /// 批量注册多个实例
    pub fn register_batch<F>(configs: Vec<DbConfig>, factory: F) -> Result<Vec<Arc<Self>>, Box<dyn Error>>
    where
        F: Fn(&DbConfig) -> Result<M, DatabaseError> + Sync + Send + 'static + Clone,
    {
        let mut instances = Vec::new();
        for config in configs {
            let instance = Self::register(&config, factory.clone())?;
            instances.push(instance);
        }
        Ok(instances)
    }

    /// 注销实例
    pub fn unregister(name: &str) -> Result<Option<Arc<Self>>, Box<dyn Error>> {
        let registry = DB_REGISTRY.get()
            .ok_or("Database registry not initialized")?;

        let removed = registry.remove::<M>(name);

        if removed.is_some() {
            info!("Database instance '{}' unregistered successfully", name);
        }

        Ok(removed)
    }

    /// 获取指定名称的实例
    pub fn get_instance(name: &str) -> Result<Arc<Self>, DatabaseError> {
        let registry = DB_REGISTRY.get().ok_or(DatabaseError::NotFoundError("Database registry not initialized".to_string()))?;
        registry.get_instance::<M>(name).ok_or(DatabaseError::NotFoundError(format!("Database instance '{}' not found", name)))
    }

    /// 获取当前数据源名称对应的实例
    pub fn get_current() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;
        let name = get_datasource_name();
        registry.get_instance::<M>(&name)
    }

    /// 获取默认实例
    pub fn default() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;
        registry.get_default::<M>()
    }

    /// 检查实例是否存在
    pub fn exists(name: &str) -> bool {
        if let Some(registry) = DB_REGISTRY.get() {
            registry.contains::<M>(name)
        } else {
            false
        }
    }

    /// 列出所有已注册的实例名称
    pub fn list_instances() -> Vec<String> {
        if let Some(registry) = DB_REGISTRY.get() {
            registry.list_instances::<M>()
        } else {
            Vec::new()
        }
    }

    /// 获取实例总数
    pub fn count() -> usize {
        if let Some(registry) = DB_REGISTRY.get() {
            registry.instance_count()
        } else {
            0
        }
    }

    /// 获取实例元数据
    // pub fn get_metadata(name: &str) -> Option<InstanceMetadata> {
    //     if let Some(registry) = DB_REGISTRY.get() {
    //         registry.get_metadata(name)
    //     } else {
    //         None
    //     }
    // }

    /// 获取连接池的引用
    #[inline]
    pub fn get_pool(&self) -> &M {
        &self.pool
    }

    /// 获取数据库类型
    #[inline]
    pub fn get_db_type(&self) -> DbType {
        self.db_type
    }

    /// 获取实例名称
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

// ==================== 扩展 trait 用于批量操作 ====================

pub trait DatabaseManagerExt<M: Send + Sync + 'static> {
    fn with_all_instances<F, R>(f: F) -> Vec<R>
    where
        F: Fn(Arc<DbManager<M>>) -> R + Send + Sync;
}

impl<M: Send + Sync + 'static> DatabaseManagerExt<M> for DbManager<M> {
    fn with_all_instances<F, R>(f: F) -> Vec<R>
    where
        F: Fn(Arc<DbManager<M>>) -> R + Send + Sync,
    {
        let registry = match DB_REGISTRY.get() {
            Some(r) => r,
            None => return Vec::new(),
        };

        let type_id = TypeId::of::<M>();
        registry.instances
            .iter()
            .filter(|entry| entry.key().1 == type_id)
            .filter_map(|entry| {
                entry.value().clone().downcast::<DbManager<M>>().ok().map(|instance| f(instance))
            })
            .collect()
    }
}

pub trait DbRegister{
    fn register_db(&self,config: &DbConfig) -> Result<(), DatabaseError>;

    fn check_config(&self, config: &DbConfig) -> Result<(), DatabaseError>{
        if config.database.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Database is missing".to_string()));
        }
        if config.username.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Username is missing".to_string()));
        }
        if config.password.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Password is missing".to_string()));
        }
        if config.host.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Host is missing".to_string()));
        }
        if config.port.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Port is missing".to_string()));
        }
        Ok(())
    }

}