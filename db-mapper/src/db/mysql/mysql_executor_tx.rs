use std::marker::PhantomData;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::Executor;
use r2d2_mysql::mysql::Transaction;
use std::sync::{Arc, Mutex};
use tokio::task_local;
use crate::sql::executor_tx::ExecutorTx;

task_local! {
    pub static TX_REGISTER : Arc<Mutex<Transaction>>;
}
pub const MYSQL_SQL_EXECUTOR_TX: MysqlSqlExecutorTx = MysqlSqlExecutorTx { _e: PhantomData };

pub struct MysqlSqlExecutorTx<'a>{
    _e:PhantomData<&'a ()>,
}

impl<'a> ExecutorTx for MysqlSqlExecutorTx<'a>{
    type Tx = Transaction<'a>;

    fn query_some_tx<E>(&self, tx: &Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_one_tx<E>(&self, tx: &Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_count_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn insert_tx<E>(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn insert_batch_tx<E>(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn delete_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn update_tx(&self, tx: &mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }
}

