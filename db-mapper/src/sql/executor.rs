use crate::base::entity::Entity;

use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

use std::option::Option;

pub(crate) trait Executor{
    type T;

    fn get_sql_executor() -> &'static Self;

    fn query_some<E>(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;

    // 查询单个结果
    fn query_one<E>(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;

    fn query_count(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;
    // 执行插入操作，返回主键
    fn insert<E>(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<E::K,DatabaseError>where E:Entity;

    fn insert_batch<E>(&self, tx: &Self::T, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> where E: Entity;

    fn delete(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

    fn update(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

    fn start_transaction(&self, tx:&Self::T) -> Result<(), DatabaseError>;

    fn commit(&self, tx:&Self::T) -> Result<(),DatabaseError>;

    fn rollback(&self, tx:&Self::T) -> Result<(),DatabaseError>;

    fn exec_tx(&self, tx:&Self::T) -> Result<(),DatabaseError>;
}

#[macro_export]
macro_rules! exec_tx {
    (
        $sql:expr,
        $params: expr,
        $f: tt
    ) => {{
        use crate::sql::executor::Executor;
        use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
        use crate::pool::datasource::get_datasource_type;
        use r2d2_mysql::MySqlConnectionManager;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use r2d2_mysql::mysql::TxOpts;

        let result = task::spawn_blocking(move || -> Result<_, DatabaseError> {
            let db_type_opt = get_datasource_type();
            let db_type = db_type_opt.ok_or(DatabaseError::NotFoundError(
                "DataSource Not config !!!".to_string()
            ))?;

            match db_type {
                DbType::Mysql => {
                    // 获取连接管理器
                    let manager = DbManager::get_instance().ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

                    // 获取连接
                    let mut conn: PooledConnection<MySqlConnectionManager> = manager.get_conn()?;

                    // 开始事务
                    let tx = conn.start_transaction(TxOpts::default())
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;

                    // 执行查询
                    MysqlSqlExecutor::get_sql_executor()
                        .$f(&tx, $sql, $params)
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }
        })
            .await;  // 这里返回 Result<Result<Option<E>, DatabaseError>, JoinError>

        // 处理两层 Result
        match result {
            Ok(query_result) => query_result,  // 内层 Result<Option<E>, DatabaseError>
            Err(join_error) => Err(DatabaseError::CommonError(format!("Task execution failed: {}", join_error))),
        }
    }};

    (
        $sql:expr,
        $params: expr,
        $e: ident,
        $f: tt
    ) => {{
                        use crate::sql::executor::Executor;
        use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
        use crate::pool::datasource::get_datasource_type;
        use r2d2_mysql::MySqlConnectionManager;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use r2d2_mysql::mysql::TxOpts;

        // 使用 tokio::task::spawn_blocking 执行阻塞操作
        let result = task::spawn_blocking(move || -> Result<_, DatabaseError> {
            let db_type_opt = get_datasource_type();
            let db_type = db_type_opt.ok_or(DatabaseError::NotFoundError(
                "DataSource Not config !!!".to_string()
            ))?;

            match db_type {
                DbType::Mysql => {
                    // 获取连接管理器
                    let manager = DbManager::get_instance().ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

                    // 获取连接
                    let mut conn: PooledConnection<MySqlConnectionManager> = manager.get_conn()?;

                    // 开始事务
                    let tx = conn.start_transaction(TxOpts::default())
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;

                    // 执行查询
                    MysqlSqlExecutor::get_sql_executor()
                        .$f::<E>(&tx, $sql, $params)
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }
        })
            .await;  // 这里返回 Result<Result<Option<E>, DatabaseError>, JoinError>

        // 处理两层 Result
        match result {
            Ok(query_result) => query_result,  // 内层 Result<Option<E>, DatabaseError>
            Err(join_error) => Err(DatabaseError::CommonError(format!("Task execution failed: {}", join_error))),
        }
    }};
}