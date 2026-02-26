use crate::base::entity::Entity;

use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

use std::option::Option;
use crate::base::db_type::DbType;
use crate::pool::datasource::get_datasource_type;

#[macro_export]
macro_rules! exec_tx_with {
    // 模式1: 无实体类型参数
    ($tx:expr, $db_type:expr,$sql:expr, $params:expr, $f:tt) => {
        exec_tx_with!(@inner $tx, $db_type, $sql, $params, $f,)
    };

    // 模式2: 有实体类型参数
    ($tx:expr, $db_type:expr,$sql:expr, $params:expr, $e:ident, $f:tt) => {
        exec_tx_with!(@inner $tx, $db_type, $sql, $params, $f, $e)
    };

    // 内部实现 - 统一处理
    (@inner $tx:expr, $db_type:expr, $sql:expr, $params:expr, $f:tt, $($type_args:tt)?) => {{
        // 提前导入所有依赖
        use tokio::task;
        use crate::sql::executor::Executor;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use rustlog::{info};

        info!("Executing sql [{}] params[{:?}]", $sql, $params);

        // 创建闭包执行数据库操作

            match $db_type {
                DbType::Mysql => {
                    // // mysql
                    use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
                    use r2d2_mysql::MySqlConnectionManager;
                    use r2d2_mysql::mysql::TxOpts;

                    // let tx:&r2d2_mysql::mysql::Transaction = $tx as &r2d2_mysql::mysql::Transaction;
                    // 执行查询 - 根据是否有类型参数选择调用方式
                    let res = MysqlSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? ($tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                DbType::Sqlite => {
                    use crate::db::sqlite::sqlite_executor::SqliteSqlExecutor;
                    use r2d2_sqlite::SqliteConnectionManager;

                    // let tx:&rusqlite::Transaction = $tx as &rusqlite::Transaction;
                    // 执行查询 - 根据是否有类型参数选择调用方式
                    let res = SqliteSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? ($tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }
        }};
}

#[macro_export]
macro_rules! exec_tx {
    // 模式1: 无实体类型参数
    ($db_type:expr,$sql:expr, $params:expr, $f:tt) => {
        exec_tx!(@inner $db_type, $sql, $params, $f,)
    };

    // 模式2: 有实体类型参数
    ($db_type:expr,$sql:expr, $params:expr, $e:ident, $f:tt) => {
        exec_tx!(@inner $db_type, $sql, $params, $f, $e)
    };

    // 内部实现 - 统一处理
    (@inner $db_type:expr, $sql:expr, $params:expr, $f:tt, $($type_args:tt)?) => {{
        // 提前导入所有依赖
        use tokio::task;
        use crate::sql::executor::Executor;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use rustlog::{info};

        info!("Executing sql [{}] params[{:?}]", $sql, $params);

        // 创建闭包执行数据库操作
            match $db_type {
                DbType::Mysql => {
                    // mysql
                    use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
                    use r2d2_mysql::MySqlConnectionManager;
                    use r2d2_mysql::mysql::TxOpts;
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
                    let res = MysqlSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? (&tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    tx.commit().map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                DbType::Sqlite => {
                    use crate::db::sqlite::sqlite_executor::SqliteSqlExecutor;
                    use r2d2_sqlite::SqliteConnectionManager;
                    // 获取连接管理器
                    let manager = DbManager::get_instance()
                        .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

                    // 获取连接
                    let mut conn: PooledConnection<SqliteConnectionManager> = manager.get_conn()
                        .map_err(|e| DatabaseError::CommonError(e.to_string()))?;

                    // 开始事务
                    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;

                    // 执行查询 - 根据是否有类型参数选择调用方式
                    let res = SqliteSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? (&tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    tx.commit().map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }

    }};
}


#[macro_export]
macro_rules! exec {
    // 模式1: 无实体类型参数
    ($sf:tt, $f:tt) => {
        exec_tx!(@inner $sf, $f,)
    };

    // 模式2: 有实体类型参数
    ($db_type:expr,$sql:expr, $params:expr, $e:ident, $f:tt) => {
        exec_tx!(@inner $sf, $f, $e)
    };

    // 内部实现 - 统一处理
    (@inner $sf:tt, $f:tt, $($type_args:tt)?) => {{
        // 提前导入所有依赖
        use tokio::task;
        use crate::sql::executor::Executor;
        use crate::pool::db_manager::DbManager;
        use r2d2::PooledConnection;
        use rustlog::{info};

        info!("Executing sql [{}] params[{:?}]", $sql, $params);
        // 获取sql和参数
        let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
            "datasource type is null".to_string(),
        ))?;
        let (sql, param_vec) = db_type.gen_select_by_key_sql::<E>(key.clone());
        // 创建闭包执行数据库操作
            match db_type {
                DbType::Mysql => {
                    // mysql
                    use crate::db::mysql::mysql_executor::MysqlSqlExecutor;
                    use r2d2_mysql::MySqlConnectionManager;
                    use r2d2_mysql::mysql::TxOpts;
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
                    let res = MysqlSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? (&tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    tx.commit().map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                DbType::Sqlite => {
                    use crate::db::sqlite::sqlite_executor::SqliteSqlExecutor;
                    use r2d2_sqlite::SqliteConnectionManager;
                    // 获取连接管理器
                    let manager = DbManager::get_instance()
                        .ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;

                    // 获取连接
                    let mut conn: PooledConnection<SqliteConnectionManager> = manager.get_conn()
                        .map_err(|e| DatabaseError::CommonError(e.to_string()))?;

                    // 开始事务
                    let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)
                        .map_err(|e| DatabaseError::CommonError(format!("Failed to start transaction: {}", e)))?;

                    // 执行查询 - 根据是否有类型参数选择调用方式
                    let res = SqliteSqlExecutor::get_sql_executor()
                        .$f $(::<$type_args>)? (&tx, $sql, $params)
                        .map_err(|e| DatabaseError::CommonError(e.to_string()));
                    tx.commit().map_err(|e| DatabaseError::CommonError(e.to_string()));
                    res
                },
                _ => Err(DatabaseError::NotFoundError("Database type not supported".to_string()))
            }

    }};
}

// pub fn do_exec<SF,QF,Tx,Q>()->Result<Q,DatabaseError> where SF:Fn(DbType)->(String,Vec<ParamValue>), QF:Fn(&Tx,String,Vec<ParamValue>)->Result<Q, DatabaseError> {
//     let db_type = get_datasource_type().ok_or(DatabaseError::NotFoundError(
//         "datasource type is null".to_string(),
//     ))?;
//     let (sql,param_vec) = SF(db_type);
//     let tx = db_type.start_transaction();
//     QF(&tx,sql,param_vec)
// }

pub(crate) trait Executor{
    
    fn query_some<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity;

    // 查询单个结果
    fn query_one<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity;

    fn query_count(&self, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;
    // 执行插入操作，返回主键
    fn insert<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E::K>,DatabaseError>where E:Entity;

    fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> where E: Entity;

    fn delete(&self, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

    fn update(&self, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

    fn start_transaction(&self) -> Result<(), DatabaseError>;
    
    fn commit(&self) -> Result<(),DatabaseError>;
    
    fn rollback(&self) -> Result<(),DatabaseError>;

}
