use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::Executor;
use std::sync::OnceLock;
// 定义线程本地存储的 HashMap（每个线程一个独立副本）
// 定义线程本地存储的 HashMap（每个线程一个独立副本）

pub struct MysqlSqlExecutor;

static MYSQL_SQL_EXECUTOR_CONFIG: OnceLock<MysqlSqlExecutor> = OnceLock::new();

impl Executor for MysqlSqlExecutor{

    fn get_sql_executor() -> &'static Self {
        MYSQL_SQL_EXECUTOR_CONFIG.get_or_init(||MysqlSqlExecutor)
    }

    async fn query_some<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn query_one<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    async fn insert(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Option<ParamValue>, DatabaseError>
    {
        todo!()
    }
}

