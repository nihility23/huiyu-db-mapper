use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

pub(crate) trait Executor{
    fn get_sql_executor()->&'static Self;

    async fn query_some<E>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;
    async fn query_one<E>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;
    async fn exec<E,T>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<T,DatabaseError> where E:Entity;
    async fn query_count<E,T>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<T,DatabaseError> where E:Entity;
    async fn insert<E,T>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<T,DatabaseError> where E:Entity;
    async fn update<E,T>(&self, db_name: Option<&str>, sql:&str, params: &Vec<ParamValue>) -> Result<T,DatabaseError> where E:Entity;
}

