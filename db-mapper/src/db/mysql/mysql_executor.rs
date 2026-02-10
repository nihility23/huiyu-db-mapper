use std::marker::PhantomData;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::Executor;
use std::sync::OnceLock;
use r2d2_mysql::mysql::{Transaction, TxOpts};


// 定义线程本地存储的 HashMap（每个线程一个独立副本）
// 定义线程本地存储的 HashMap（每个线程一个独立副本）
pub const MYSQL_SQL_EXECUTOR: MysqlSqlExecutor<'static> = MysqlSqlExecutor { _a: PhantomData };

pub struct MysqlSqlExecutor<'a>{
    _a: PhantomData<&'a ()>,
}

impl<'a> Executor for MysqlSqlExecutor<'a>{
    type T = Transaction<'a>;

    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    async fn query_some<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn query_one<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn query_count(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    {
        todo!()
    }

    async fn insert<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<E::K, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn delete(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    async fn update(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }


    async fn start_transaction(&self, tx: &Self::T) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn commit(&self, tx: &Self::T) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn rollback(&self, tx: &Self::T) -> Result<(), DatabaseError> {
        todo!()
    }

    async fn exec_tx(&self, tx: &Self::T) -> Result<(), DatabaseError> {
        todo!()
    }

    // async fn query_some_tx<E,T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    // where
    //     E: Entity
    // {
    //     todo!()
    // }
    //
    // async fn query_one_tx<E,T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    // where
    //     E: Entity
    // {
    //     todo!()
    // }

    // async fn insert_tx<T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>, DatabaseError>
    // {
    //     todo!()
    // }

    // async fn start_transaction(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn commit(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // async fn rollback(&self) -> Result<(), DatabaseError> {
    //     todo!()
    // }
}

