// use r2d2::{ManageConnection, Pool, PooledConnection};
// use std::any::{Any, TypeId};
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::sync::{Arc, OnceLock, RwLock};
//
// thread_local! {
//     static TX_ID_MAP: RefCell<String> = RefCell::new(String::new());
// }
//
// type RegistryMap = HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>;
// static DB_REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();
// static TX_REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();
// pub struct DbManager<T:ManageConnection>{
//     /// 核心数据存储
//     pool_data: Arc<RwLock<Pool<T>>>,
//
//     /// 实例标签
//     name: String,
// }
//
// pub struct TxManager<T:ManageConnection>{
//     tx_data: Arc<RwLock<PooledConnection<T>>>,
//     tx_id: String,
// }
//
// impl<T:ManageConnection> TxManager<T>{
//     pub fn get_instance(db_name: String,tx_id: String)->Arc<Self>{
//         // 类型别名，提高代码可读性
//         type RegistryMap = HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>;
//
//         static REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();
//
//         let registry = REGISTRY.get_or_init(|| RwLock::new(RegistryMap::new()));
//         let key = (tx_id.to_string(), TypeId::of::<T>());
//
//         // 快速路径：读锁查找
//         {
//             let registry_guard = registry.read().unwrap();
//             if let Some(existing) = registry_guard.get(&key) {
//                 if let Ok(typed) = existing.clone().downcast::<Self>() {
//                     return typed;
//                 }
//             }
//         }
//
//         // 慢路径：创建新实例
//         let mut registry_guard = registry.write().unwrap();
//
//         // 双重检查
//         if let Some(existing) = registry_guard.get(&key) {
//             if let Ok(typed) = existing.clone().downcast::<Self>() {
//                 return typed;
//             }
//         }
//
//         DbManager::instance(&db_name);
//
//         // 创建并注册
//         let instance = Arc::new(Self {
//             tx_data: Arc::new(RwLock::new(initializer())),
//             tag: tag.to_string(),
//         });
//
//         registry_guard.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);
//         instance
//     }
// }
//
// impl<T: ManageConnection> DbManager<T> {
//     /// 获取或创建简化单例（实现与完整版类似但更简洁）
//     pub fn instance<F>(tag: &str, initializer: F) -> Arc<Self>
//     where
//         F: FnOnce() -> Pool<T>,
//     {
//         // 类型别名，提高代码可读性
//         type RegistryMap = HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>;
//
//         static REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();
//
//         let registry = REGISTRY.get_or_init(|| RwLock::new(RegistryMap::new()));
//         let key = (tag.to_string(), TypeId::of::<T>());
//
//         // 快速路径：读锁查找
//         {
//             let registry_guard = registry.read().unwrap();
//             if let Some(existing) = registry_guard.get(&key) {
//                 if let Ok(typed) = existing.clone().downcast::<Self>() {
//                     return typed;
//                 }
//             }
//         }
//
//         // 慢路径：创建新实例
//         let mut registry_guard = registry.write().unwrap();
//
//         // 双重检查
//         if let Some(existing) = registry_guard.get(&key) {
//             if let Ok(typed) = existing.clone().downcast::<Self>() {
//                 return typed;
//             }
//         }
//
//         // 创建并注册
//         let instance   = Arc::new(Self {
//             pool_data: Arc::new(RwLock::new(initializer())),
//             tag: tag.to_string(),
//         });
//
//         registry_guard.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);
//         instance
//     }
//
//     /// 获取数据（克隆版本）
//     pub fn get(&self) -> Pool<T>
//     where
//         T: Clone,
//     {
//         self.data.read().unwrap().clone()
//     }
// }
// // pub(crate) struct DbManager<T: ManageConnection> {
// //     pool_map: HashMap<String,Pool<T>>,
// //     tx_map: HashMap<String,PooledConnection<T>>
// // }
// //
// // // 连接是获取所有权
// // // pool获取副本
// // impl <T:ManageConnection> DbManager<T>{
// //     pub(crate) fn get_conn(&mut self, db_name: Option<String>, tx_id_opt : Option<String>)->PooledConnection<T>{
// //         if tx_id_opt.is_some(){
// //             let tx_id = tx_id_opt.unwrap();
// //             let v = self.tx_map.remove(&tx_id);
// //             if let Some(v) = v{
// //                 return v;
// //             }
// //         }
// //         let p = self.pool_map.get(&db_name.unwrap());
// //         let conn = p.unwrap().get().unwrap();
// //         conn
// //     }
// //
// //     pub(crate) fn store_tx_conn(&mut self, tx_id : Option<String>, conn: PooledConnection<T>){
// //         self.tx_map.insert(tx_id.unwrap(), conn);
// //     }
// //
// //     pub(crate) fn release_conn(&mut self, db_name: Option<String>, conn: PooledConnection<T>){
// //         // drop自动归还
// //     }
// //
// //     pub(crate) fn init(&self, db_configs: &Vec<DbConfig>){
// //         for db_config in db_configs {
// //             match db_config.db_type{
// //                 base::db_type::DbType::Mysql=>{
// //                     let db_manager:&DbManager<MySqlConnectionManager> = DbManager::get_instance();
// //                     // db_manager.pool_map.insert(db_config.name.unwrap(),)
// //                 }
// //                 base::db_type::DbType::Oracle=>{
// //                     let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
// //                 }
// //                 base::db_type::DbType::Postgres=>{
// //                     let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
// //                 }
// //                 base::db_type::DbType::Sqlite=>{
// //                     let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
// //                 }
// //                 base::db_type::DbType::SqlServer=>{
// //                     let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
// //                 }
// //             }
// //         }
// //     }
// //
// //     pub fn get_instance() -> &'static mut DbManager<T> {
// //         /// 静态存储单例实例的容器
// //         // static INSTANCE: OnceLock<&'static mut DbManager> = OnceLock::new();
// //         Box::leak(Box::new(DbManager { pool_map: HashMap::new(), tx_map: HashMap::new() }))
// //     }
// // }
// //
// // fn test(){
// //     let db_manager:&DbManager<SqliteConnectionManager> = DbManager::get_instance();
// //     db_manager.init(&Vec::new());
// //     let db_manager:&DbManager<MySqlConnectionManager> = DbManager::get_instance();
// //     db_manager.init(&Vec::new());
// // }