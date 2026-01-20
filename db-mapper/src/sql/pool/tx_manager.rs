// use crate::sql::pool::db_manager::DbManager;
// use r2d2_mysql::mysql::prelude::WithParams;
// use r2d2_mysql::mysql::Transaction;
// use std::any::{Any, TypeId};
// use std::cell::RefCell;
// use std::collections::HashMap;
// use std::sync::{Arc, OnceLock, RwLock};
// use uuid::Uuid;
// use crate::base::error::DatabaseError;
// use crate::base::error::DatabaseError::BusinessError;
//
// thread_local! {
//     static TX_ID_MAP: RefCell<Option<String>> = RefCell::new(None);
// }
//
// type RegistryMap = HashMap<(String, TypeId), Arc<dyn Any + Send + Sync>>;
// static TX_REGISTRY: OnceLock<RwLock<RegistryMap>> = OnceLock::new();
//
// pub struct TxManager<'a>{
//     tx_data: Arc<RwLock<Transaction<'a>>>,
//     tx_id: String,
// }
//
// impl<'a> TxManager<'a>{
//     pub fn get_tx_id()->String{
//         TX_ID_MAP.with(|tx| {
//             tx.borrow_mut().clone().unwrap_or(Uuid::new_v4().to_string())
//         })
//     }
//     pub fn get_instance<T>(db_name: &str)->Result<Option<Arc<Self>>,DatabaseError>{
//
//         let registry = TX_REGISTRY.get_or_init(|| RwLock::new(RegistryMap::new()));
//         let tx_id = Self::get_tx_id();
//         let key = (tx_id.to_string(), TypeId::of::<Transaction>());
//
//         // 快速路径：读锁查找
//         {
//             let registry_guard = registry.read().unwrap();
//             if let Some(existing) = registry_guard.get(&key) {
//                 if let Ok(typed) = existing.clone().downcast::<Self>() {
//                     return Ok(Some(typed));
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
//                 return Ok(Some(typed));
//             }
//         }
//
//         let db_manager_opt = DbManager::get_instance(&db_name);
//         if db_manager_opt.is_none() {
//             return Err(DatabaseError::BusinessError("Can't get datasource instance".to_string()));
//         }
//
//         let db_manager = db_manager_opt.unwrap();
//         let conn = db_manager.get().get()?;
//
//         // 创建并注册
//         let instance = Arc::new(Self {
//             tx_data: Arc::new(RwLock::new(conn)),
//             tx_id: tx_id.to_string(),
//         });
//
//         registry_guard.insert(key, instance.clone() as Arc<dyn Any + Send + Sync>);
//         Ok(instance)
//     }
// }
