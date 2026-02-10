use crate::base::entity::Entity;
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
        let db_type_opt = get_datasource_type();
        let db_type = db_type_opt.ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;
        let param_vec = $params.clone();
        async {
            blocking::unblock(move || {
                match db_type {
                    DbType::Mysql => {
                        let mut conn: PooledConnection<MySqlConnectionManager> = DbManager::get_instance().unwrap().get_conn()?;
                        let tx = conn.start_transaction(TxOpts::default()).unwrap();
                        MysqlSqlExecutor::get_sql_executor().$f(&tx, $sql, &param_vec)
                    },
                    _ => panic!("Not support")
                }
            }).await
        }.await
    }};
    ($tx:expr, $f: expr)=>{{
        let db_type_opt = get_datasource_type();
        let db_type = db_type_opt.ok_or(DatabaseError::NotFoundError("DataSource Not config !!!".to_string()))?;
        match db_type {
            DbType::Mysql => {
                return MysqlSqlExecutor::get_sql_executor().$f($tx).await;
            },
            _ => panic!("Not support")
        }
    }};
}
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
     fn delete(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

     fn update(&self, tx:&Self::T, sql:&str, params: &Vec<ParamValue>) -> Result<u64,DatabaseError>;

     fn start_transaction(&self, tx:&Self::T) -> Result<(), DatabaseError>;

     fn commit(&self, tx:&Self::T) -> Result<(),DatabaseError>;

     fn rollback(&self, tx:&Self::T) -> Result<(),DatabaseError>;

     fn exec_tx(&self, tx:&Self::T) -> Result<(),DatabaseError>;
}

