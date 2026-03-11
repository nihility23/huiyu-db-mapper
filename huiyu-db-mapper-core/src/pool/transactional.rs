// use std::cell::RefCell;
// use r2d2_mysql::mysql::Transaction;
// use tokio::task_local;
// use crate::base::db_type::DbType;
//
// pub enum TransactionType<'a> {
//     MySQL(r2d2_mysql::mysql::Transaction<'a>),
//     SQLite(rusqlite::Transaction<'a>),
// }
//
// impl<'a> TransactionType<'a> {
//     pub fn get_sqlite_tx(&'a self)->&'a r2d2_sqlite::rusqlite::Transaction<'a>{
//         match self {
//             TransactionType::SQLite(tx) => tx,
//             _ => panic!("Not a SQLite transaction"),
//         }
//     }
//
//     pub fn get_mysql_tx(&'a self)->&'a r2d2_mysql::mysql::Transaction<'a>{
//         match self {
//             TransactionType::MySQL(tx) => tx,
//             _ => panic!("Not a MySQL transaction"),
//         }
//     }
//
//     pub fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
//         match self {
//             TransactionType::SQLite(tx) => Ok(tx.commit()?),
//             TransactionType::MySQL(tx) => Ok(tx.commit()?),
//         }
//     }
//     pub fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
//         match self {
//             TransactionType::SQLite(tx) => Ok(tx.rollback()?),
//             TransactionType::MySQL(tx) => Ok(tx.rollback()?),
//         }
//     }
// }
//
//
// task_local! {
//     pub static TX_ID_REGISTRY: RefCell<Option<String>>;
// }
//
// pub fn get_transaction_id() -> Option<String> {
//     if let Some(name) = TX_ID_REGISTRY.try_get().ok() {
//         name.borrow().clone()
//     } else {
//         None
//     }
// }
//
// pub fn set_transaction_id(tx_id: &str) {
//     TX_ID_REGISTRY
//         .try_with(|name| {
//             *name.borrow_mut() = Some(tx_id.to_string());
//         })
//         .ok();
// }
//
//
// // 定义统一的事务 trait
// pub trait TransactionLike {
//     fn commit(&self) -> Result<(), Box<dyn std::error::Error>>;
//     fn rollback(&self) -> Result<(), Box<dyn std::error::Error>>;
// }
//
// // 为 MySQL Transaction 实现
// impl TransactionLike for r2d2_mysql::mysql::Transaction<'_> {
//     fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(self.commit()?)
//     }
//
//     fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(self.rollback()?)
//     }
// }
//
// // 为 SQLite Transaction 实现
// impl TransactionLike for r2d2_sqlite::rusqlite::Transaction<'_> {
//     fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(self.commit()?)
//     }
//
//     fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(self.rollback()?)
//     }
// }
//
// impl TransactionLike for DbType{
//     fn commit(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(())
//     }
//
//     fn rollback(&self) -> Result<(), Box<dyn std::error::Error>> {
//         Ok(())
//     }
// }
//
