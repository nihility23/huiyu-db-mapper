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
    // 模式1: 无实体类型参数
    ($sql:expr, $params:expr, $f:tt) => {
        exec_tx!(@inner $sql, $params, $f,)
    };

    // 模式2: 有实体类型参数
    ($sql:expr, $params:expr, $e:ident, $f:tt) => {
        exec_tx!(@inner $sql, $params, $f, $e)
    };

    // 内部实现 - 统一处理
    (@inner $sql:expr, $params:expr, $f:tt, $($type_args:tt)?) => {{
        // 提前导入所有依赖
        use crate::sql::executor::Executor;
        use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
        use crate::pool::datasource::get_datasource_type;
        use r2d2_mysql::MySqlConnectionManager;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use r2d2_mysql::mysql::TxOpts;
        use tokio::task;

        // 创建闭包执行数据库操作
        let db_operation = move || -> Result<_, DatabaseError> {
            // 获取数据库类型
            let db_type = get_datasource_type()
                .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

            match db_type {
                DbType::Mysql => {
                    // 获取连接管理器
                    let manager = DbManager::get_instance()
                        .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

                    // 获取连接
                    let mut conn: PooledConnection<MySqlConnectionManager> = manager.get_conn()
                        .map_err(|e| DatabaseError::CommonError(e.to_string()))?;

                    // 开始事务
                    let tx = conn.start_transaction(TxOpts::default())
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;

                    // 执行查询 - 根据是否有类型参数选择调用方式
                    MysqlSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? (&tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()))
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }
        };

        // 在阻塞线程中执行并处理结果
        match task::spawn_blocking(db_operation).await {
            Ok(query_result) => query_result,
            Err(join_error) => Err(DatabaseError::CommonError(format!("Task execution failed: {}", join_error))),
        }
    }};
}