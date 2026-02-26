use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::Executor;
use r2d2_mysql::mysql::Transaction;
use std::sync::{Arc, Mutex};
use tokio::task_local;

task_local! {
    pub static TX_REGISTER : Arc<Mutex<Transaction>>;
}
pub const MYSQL_SQL_EXECUTOR: MysqlSqlExecutor = MysqlSqlExecutor;

pub struct MysqlSqlExecutor;

impl Executor for MysqlSqlExecutor{

    fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_one<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_count(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn insert<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn start_transaction(&self) -> Result<(),DatabaseError> {
        todo!()
    }

    fn commit(&self) -> Result<(), DatabaseError> {
        todo!()
    }

    fn rollback(&self) -> Result<(), DatabaseError> {
        todo!()
    }
}

