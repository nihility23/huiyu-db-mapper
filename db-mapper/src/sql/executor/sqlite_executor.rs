use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::sql::executor::executor::{Executor, SqlExecutor};
use r2d2_sqlite::SqliteConnectionManager;

type SqliteSqlExecutor = SqlExecutor<SqliteConnectionManager>;

impl Executor for SqliteSqlExecutor{
    fn get_sql_executor() -> &'static Self {
        todo!()
    }

    fn exec(sql: &str, params: &Vec<ParamValue>) -> Result<(), DatabaseError> {
        println!("Executing: {}",sql);

        Ok(())
    }
}

