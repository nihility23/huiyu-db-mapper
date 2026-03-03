use crate::base::config::DbConfig;
use crate::base::db_type::DbType;
use crate::pool::datasource::{get_datasource_name, set_datasource_type};
use r2d2::{Error, ManageConnection, Pool, PooledConnection};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::fmt;
use std::time::Duration;
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
    fn insert<T: ManageConnection>(
        &mut self,
        name: String,
        instance: Arc<DbManager<T>>
    ) {
        let type_id = TypeId::of::<T>();
        let key = (name.clone(), type_id);

        self.instances.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);

        if !self.defaults.contains_key(&type_id) {
            self.defaults.insert(type_id, instance as Arc<dyn Any + Send + Sync>);
        }
    }

    /// 获取实例
    fn get_instance<T: ManageConnection>(
        &self,
        name: &str
    ) -> Option<Arc<DbManager<T>>> {
        let key = (name.to_string(), TypeId::of::<T>());
        self.instances
            .get(&key)
            .and_then(|any| any.clone().downcast::<DbManager<T>>().ok())
    }

    /// 获取默认实例
    fn get_default<T: ManageConnection>(&self) -> Option<Arc<DbManager<T>>> {
        self.defaults
            .get(&TypeId::of::<T>())
            .and_then(|any| any.clone().downcast::<DbManager<T>>().ok())
    }
}

/// 数据库管理器
pub struct DbManager<T: ManageConnection> {
    /// 连接池
    pool: Pool<T>,
    /// 实例标签
    name: String,
    /// 数据库类型
    db_type: DbType,
}

impl<T: ManageConnection> DbManager<T> {
    // ==================== 初始化方法 ====================

    /// 初始化所有数据库实例（应在程序启动时调用一次）
    pub fn initialize<F>(configs: &Vec<DbConfig>, factory: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(&DbConfig) -> Pool<T> + Sync + Send + 'static,
        T: 'static,
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
    pub fn get_conn() -> Result<PooledConnection<T>, DatabaseError>
    where
        T: ManageConnection,
    {

        Self::get_instance().ok_or_else(|| {
            DatabaseError::NotFoundError("Database instance not found".to_string())
        })?.get_inner_conn()
    }

    // ==================== 实例获取方法 ====================

    /// 获取指定名称的实例
    pub fn get_instance() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;

        let name = get_datasource_name().unwrap_or_else(|| "default".to_string());
        

        registry.get_instance::<T>(&name)
    }

    /// 获取默认实例
    pub fn default() -> Option<Arc<Self>> {
        let registry = DB_REGISTRY.get()?;
        registry.get_default::<T>()
    }

    /// 检查是否已初始化
    pub fn is_initialized() -> bool {
        DB_REGISTRY.get().is_some()
    }

    // ==================== 连接池属性获取方法（基于 r2d2::State）====================

    /// 获取连接池的完整状态信息
    pub fn state(&self) -> r2d2::State {
        self.pool.state()
    }

    /// 获取最大连接数
    pub fn max_size(&self) -> u32 {
        self.pool.max_size() as u32
    }

    /// 获取当前总连接数（包括空闲和活跃）
    pub fn size(&self) -> u32 {
        self.pool.state().connections as u32
    }

    /// 获取空闲连接数
    pub fn idle(&self) -> u32 {
        self.pool.state().idle_connections as u32
    }

    /// 获取活跃连接数
    pub fn active(&self) -> u32 {
        (self.pool.state().connections - self.pool.state().idle_connections) as u32
    }

    /// 获取连接池使用率（活跃连接数/最大连接数）
    pub fn utilization(&self) -> f64 {
        let max = self.max_size();
        if max == 0 {
            return 0.0;
        }
        self.active() as f64 / max as f64
    }

    /// 检查连接池是否已满
    pub fn is_full(&self) -> bool {
        self.size() >= self.max_size()
    }

    /// 检查是否有空闲连接
    pub fn has_idle(&self) -> bool {
        self.idle() > 0
    }

    /// 获取连接池状态摘要
    pub fn status(&self) -> String {
        format!(
            "{}: active={}, idle={}, total={}, max={}, util={:.1}%",
            self.name,
            self.active(),
            self.idle(),
            self.size(),
            self.max_size(),
            self.utilization() * 100.0
        )
    }

    // ==================== 原有的实例方法 ====================

    /// 获取连接池
    #[inline]
    pub fn get_pool(&self) -> Pool<T> {
        self.pool.clone()
    }

    /// 获取连接池的引用
    #[inline]
    pub fn pool(&self) -> &Pool<T> {
        &self.pool
    }

    /// 获取数据库类型
    #[inline]
    pub fn get_db_type(&self) -> DbType {
        self.db_type
    }

    /// 获取连接
    #[inline]
    pub fn get_inner_conn(&self) -> Result<PooledConnection<T>, DatabaseError> {
        self.pool.get().map_err(|e| DatabaseError::from(e))
    }

    /// 获取实例名称
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// 获取连接池状态信息（保留原方法名）
    pub fn get_state(&self) -> r2d2::State {
        self.pool.state()
    }
}

// ==================== trait 实现 ====================

impl<T: ManageConnection> Clone for DbManager<T> {
    fn clone(&self) -> Self {
        Self {
            pool: self.pool.clone(),
            name: self.name.clone(),
            db_type: self.db_type,
        }
    }
}

impl<T: ManageConnection> fmt::Debug for DbManager<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DbManager")
            .field("name", &self.name)
            .field("db_type", &self.db_type)
            .field("active", &self.active())
            .field("idle", &self.idle())
            .field("size", &self.size())
            .field("max", &self.max_size())
            .field("utilization", &format!("{:.1}%", self.utilization() * 100.0))
            .finish()
    }
}

impl<T: ManageConnection> fmt::Display for DbManager<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.status())
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[derive(Debug)]
    struct MockConnection;

    #[derive(Debug)]
    struct MockConnectionManager;

    impl ManageConnection for MockConnectionManager {
        type Connection = MockConnection;
        type Error = std::io::Error;

        fn connect(&self) -> Result<Self::Connection, Self::Error> {
            Ok(MockConnection)
        }

        fn is_valid(&self, _conn: &mut Self::Connection) -> Result<(), Self::Error> {
            Ok(())
        }

        fn has_broken(&self, _conn: &mut Self::Connection) -> bool {
            false
        }
    }

    #[test]
    fn test_db_manager() {
        // 准备配置
        // 准备配置
        let configs = vec![
            DbConfig {
                host: None,
                port: None,
                database: None,
                username: None,
                password: None,
                schema: None,
                name: "main".to_string(),
                db_type: DbType::Mysql,
                // ... 其他配置
            },
            DbConfig {
                host: None,
                port: None,
                database: None,
                username: None,
                password: None,
                schema: None,
                name: "replica".to_string(),
                db_type: DbType::Mysql,
                // ... 其他配置
            },
        ];

        // 初始化
        DbManager::<MockConnectionManager>::initialize(&configs, |_| {
            Pool::builder()
                .max_size(10)
                .build(MockConnectionManager)
                .unwrap()
        }).unwrap();

        // 测试静态获取连接
        let conn_static = DbManager::<MockConnectionManager>::get_conn().unwrap();
        drop(conn_static);

        let conn_by_name = DbManager::<MockConnectionManager>::get_conn().unwrap();
        drop(conn_by_name);

        // 测试获取实例
        let main = DbManager::<MockConnectionManager>::get_instance().unwrap();

        // 测试连接池属性
        assert_eq!(main.max_size(), 10);
        assert_eq!(main.size(), 10);
        assert_eq!(main.idle(), 10);
        assert_eq!(main.active(), 0);
        assert_eq!(main.utilization(), 0.0);
        assert!(!main.is_full());
        assert!(main.has_idle());

        // 获取连接后测试
        let conn = main.get_inner_conn().unwrap();
        assert_eq!(main.active(), 1);
        assert_eq!(main.idle(), 9);
        assert_eq!(main.utilization(), 0.1);
        assert!(!main.is_full());
        assert!(main.has_idle());

        // 测试状态字符串
        println!("{}", main.status());
        println!("{:?}", main);
        println!("{}", main);

        drop(conn);

        // 多线程测试
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let conn = DbManager::<MockConnectionManager>::get_conn().unwrap();
                    println!("Thread {} got connection", i);
                    thread::sleep(Duration::from_millis(10));
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}