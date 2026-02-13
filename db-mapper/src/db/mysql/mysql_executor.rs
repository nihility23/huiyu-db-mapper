use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::Executor;
use r2d2_mysql::mysql::Transaction;
use std::marker::PhantomData;


// 定义线程本地存储的 HashMap（每个线程一个独立副本）
// 定义线程本地存储的 HashMap（每个线程一个独立副本）
pub const MYSQL_SQL_EXECUTOR: MysqlSqlExecutor<'static> = MysqlSqlExecutor { _a: PhantomData };

pub struct MysqlSqlExecutor<'a>{
    _a: PhantomData<&'a ()>,
}

impl<'a> Executor for MysqlSqlExecutor<'a>{
    type T = Transaction<'a>;

    fn get_sql_executor() -> &'static Self {
        &MYSQL_SQL_EXECUTOR
    }

    fn query_some<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_one<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn query_count(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    {
        todo!()
    }

    fn insert<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn insert_batch<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn delete(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    fn update(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }

    // fn start_transaction(&self) -> Result<Self::T, DatabaseError> {
    //     todo!()
    // }
    // 
    // fn commit(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    // 
    // fn rollback(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     todo!()
    // }


    // fn start_transaction(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // fn commit(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     todo!()
    // }
    //
    // fn rollback(&self, tx: &Self::T) -> Result<(), DatabaseError> {
    //     todo!()
    // }
}

