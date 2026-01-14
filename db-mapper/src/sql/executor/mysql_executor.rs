use std::cell::RefCell;
use std::collections::HashMap;
use r2d2::Pool;
use r2d2_mysql::{mysql, MySqlConnectionManager};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::Value;
use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};

// 定义线程本地存储的 HashMap（每个线程一个独立副本）
// 定义线程本地存储的 HashMap（每个线程一个独立副本）

type MysqlSqlExecutor = SqlExecutor<MySqlConnectionManager>;
impl Executor for MysqlSqlExecutor{
    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn exec<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }
}

