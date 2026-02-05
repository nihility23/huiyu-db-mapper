use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use std::option::Option;

pub(crate) trait Executor{
    type T;
    fn get_sql_executor()->&'static Self;

    async fn query_some<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;

    // 查询单个结果
    async fn query_one<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;

    // 执行插入操作，返回主键
    async fn insert(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>,DatabaseError>;

    // async fn query_some_tx<E,T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError> where E: Entity;
    // 
    // async fn query_one_tx<E,T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError> where E: Entity;
    // 
    // async fn insert_tx<T>(&self, tx:T, sql: &str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>, DatabaseError>;

    // async fn start_transaction(&self) -> Result<(), DatabaseError>;
    //
    // async fn commit(&self) -> Result<(),DatabaseError>;
    //
    // async fn rollback(&self) -> Result<(),DatabaseError>;
}

