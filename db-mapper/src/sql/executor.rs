use crate::base::entity::Entity;

use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;

use rusqlite::Row;
use std::option::Option;
use std::sync::Arc;
use deadpool_postgres::{ClientWrapper, Object};
use deadpool_sqlite::Pool;
use tokio::sync::Mutex;
use crate::db::postgres::postgres_executor::POSTGRES_CONN_REGISTER;
use crate::pool::db_manager::DbManager;

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

pub(crate) trait RowType{
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError> where Self: Sized ;

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError> where Self: Sized ;
}

pub(crate) trait Executor{
    type Row<'a>: RowType + 'a;
    type Conn: AsRef<Self::ConnWrapper>;
    type ConnWrapper;

    async fn query<T, R, F, Q>(
        &self,
        conn: &Self::ConnWrapper,
        sql: String,
        params: Vec<ParamValue>,
        mapper: F,
        processor: Q,
    ) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static;
    async fn execute(
        &self,
        conn: &Self::ConnWrapper,
        sql: String,
        params: Vec<ParamValue>,
    ) -> Result<u64, DatabaseError>;


    async fn exec_basic(&self, sql: String, params: Vec<ParamValue>) -> Result<u64, DatabaseError> {
        let conn_ref = self.get_conn_ref();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            let conn = conn_ref.lock().await;
            let conn = conn.as_ref();
            self.execute(conn, sql, params).await
        } else {
            let conn: Self::Conn = self.get_conn().await;
            self.execute(conn.as_ref(), sql, params).await
        }
    }


    async fn query_basic<T, R, F, Q>(
        &self,
        sql: String,
        params: Vec<ParamValue>,
        mapper: F,
        processor: Q,
    ) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static{

        let conn_ref = self.get_conn_ref();
        if conn_ref.is_ok() {
            let conn_ref = conn_ref.unwrap().clone();
            let conn = conn_ref.lock().await;
            let conn = conn.as_ref();
            self.query(conn, sql, params, mapper, processor).await // 现在可以借用
        } else {
            let conn = self.get_conn().await;
            self.query(conn.as_ref(), sql, params, mapper, processor).await
        }
    }

    fn row_to_e<E>(row: &Self::Row<'_>) -> Result<E, DatabaseError> where E:Entity;

    fn get_conn_ref(&self)-> Result<Arc<Mutex<Self::Conn>>,DatabaseError> ;

    async fn get_conn(&self)-> Self::Conn;

    async fn query_some<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Vec<E>,DatabaseError> where E:Entity{
        self.query_basic::<E, Vec<E>, _, _>(sql.to_string(), params.to_vec(), |row|Self::row_to_e(row), |results: Vec<E>| {
            Ok(results)
        }).await
    }

    // 查询单个结果
    async fn query_one<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E>,DatabaseError> where E:Entity{
        {
            self.query_basic::<E, Option<E>, _, _>(sql.to_string(), params.to_vec(), |row|Self::row_to_e(row), |results: Vec<E>| {
                Ok(results.into_iter().next())
            }).await
        }
    }

    async fn query_count(&self, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>{
        self.query_basic::<i64, u64, _, _>(
            sql.to_string(),
            params.to_vec(),
            |row| {
                let v = (row).col_to_v_by_index(0).unwrap();
                Ok(v.into())
            },
            |results: Vec<i64>| Ok(results[0] as u64),
        ).await
    }
    // 执行插入操作，返回主键
    async fn insert<E>(&self, sql:&str, params: &Vec<ParamValue>) -> Result<Option<E::K>,DatabaseError>where E:Entity{
        self.query_basic::<ParamValue, Option<E::K>, _, _>(
            sql.to_string(),
            params.to_vec(),
            |row| {
                let val = (row).col_to_v_by_index(0);
                match val {
                    Ok(v) => return Ok(v),
                    Err(e) => return Ok(ParamValue::Null),
                }
            },
            |results: Vec<ParamValue>| {
                if results.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(results[0].clone().into()))
                }
            },
        ).await
    }

     async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity,
    {
        self.exec_basic(sql.to_string(), params.clone()).await
    }

    async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        self.exec_basic(sql.to_string(), params.clone()).await
    }

    async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        self.exec_basic(sql.to_string(), params.clone()).await
    }

}
