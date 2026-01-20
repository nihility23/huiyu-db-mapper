use std::cell::RefCell;
use std::collections::HashMap;
use r2d2::Pool;
use r2d2_mysql::{mysql, MySqlConnectionManager};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::types::Value;
use crate::base::entity::Entity;
use crate::base::error::{DatabaseError, RowError};
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};

// 定义线程本地存储的 HashMap（每个线程一个独立副本）
// 定义线程本地存储的 HashMap（每个线程一个独立副本）

type MysqlSqlExecutor = SqlExecutor<MySqlConnectionManager>;
impl Executor for MysqlSqlExecutor{
    type R<'a> = ();

    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn row_to_entity<'a, E>(row: &Self::R<'_>) -> Result<E, RowError>
    where
        E: Entity
    {
        todo!()
    }

    fn exec<E>(&self, db_name: Option<&str>, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }
}

