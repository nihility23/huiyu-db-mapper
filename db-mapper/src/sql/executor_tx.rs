use crate::base::entity::Entity;

use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

use std::option::Option;
use crate::base::db_type::DbType;
use crate::db::sqlite::sqlite_executor_tx::SQLITE_SQL_EXECUTOR_TX;

pub trait ExecutorTx{

    type Tx;

    fn query_some_tx<E>(&self,tx:&Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;

    // 查询单个结果
    fn query_one_tx<E>(&self,tx:& Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;

    fn query_count_tx(&self,tx:&mut Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;
    // 执行插入操作，返回主键
    fn insert_tx<E>(&self,tx:&mut Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E::K>,DatabaseError>where E:Entity;

    fn insert_batch_tx<E>(&self,tx:&mut Self::Tx, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> where E: Entity;

    fn delete_tx(&self,tx:&mut Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

    fn update_tx(&self,tx:&mut Self::Tx, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

}

