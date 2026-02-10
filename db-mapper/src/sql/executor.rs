use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use std::option::Option;

pub(crate) trait Executor<'a>{
    type T;

    fn get_sql_executor() -> &'a Self;
    
    async fn query_some<E>(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;

    // 查询单个结果
    async fn query_one<E>(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;

    // 执行插入操作，返回主键
    async fn insert(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>,DatabaseError>;
    
    async fn start_transaction(&self, tx:&Self::T) -> Result<(), DatabaseError>;
    
    async fn commit(&self, tx:&Self::T) -> Result<(),DatabaseError>;
    
    async fn rollback(&self, tx:&Self::T) -> Result<(),DatabaseError>;
    
    async fn exec_tx(&self, tx:&Self::T) -> Result<(),DatabaseError>;
}

