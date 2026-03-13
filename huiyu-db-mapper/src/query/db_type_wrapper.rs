use std::sync::Arc;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_mapper_core::base::entity::{Entity};
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::pool::db_manager::DbRegister;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};
#[cfg(feature = "mysql")]
use huiyu_db_mapper_mysql::mysql::mysql_executor::MYSQL_SQL_EXECUTOR;
use huiyu_db_mapper_mysql::mysql::mysql_register::MYSQL_DB_REGISTER;
#[cfg(feature = "mysql")]
use huiyu_db_mapper_mysql::mysql::mysql_sql_generator::MYSQL_SQL_GENERATOR;
#[cfg(feature = "postgres")]
use huiyu_db_mapper_postgres::postgres::postgres_executor::POSTGRES_SQL_EXECUTOR;
use huiyu_db_mapper_postgres::postgres::postgres_register::POSTGRES_DB_REGISTER;
#[cfg(feature = "postgres")]
use huiyu_db_mapper_postgres::postgres::postgres_sql_generator::POSTGRES_SQL_GENERATOR;
#[cfg(feature = "sqlite")]
use huiyu_db_mapper_sqlite::sqlite::sqlite_executor::SQLITE_SQL_EXECUTOR;
use huiyu_db_mapper_sqlite::sqlite::sqlite_register::SQLITE_DB_REGISTER;
#[cfg(feature = "sqlite")]
use huiyu_db_mapper_sqlite::sqlite::sqlite_sql_generator::SQLITE_SQL_GENERATOR;

macro_rules! impl_db_method_generic {
    ($method:ident($($param:ident: $param_type:ty),*) -> $ret:ty) => {
        fn $method(&self, $($param: $param_type),*) -> $ret {
            match self.0 {
                #[cfg(feature = "mysql")]
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "postgres")]
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                // DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "sqlite")]
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                // DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
                _ => {panic!()}
            }
        }
    };
    // 模式1: 单个泛型参数，有 where 子句
    ($method:ident <$g:ident> ($($param:ident: $param_type:ty),*) -> $ret:ty where $($where_clause:tt)+) => {
        fn $method <$g> (&self, $($param: $param_type),*) -> $ret
        where $($where_clause)+
        {
            match self.0 {
                #[cfg(feature = "mysql")]
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "postgres")]
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                // DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "sqlite")]
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                // DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
                _ => {panic!()}
            }
        }
    };

    // 模式2: 单个泛型参数带约束，有 where 子句
    ($method:ident <$g:ident: $bound:path> ($($param:ident: $param_type:ty),*) -> $ret:ty where $($where_clause:tt)+) => {
        fn $method <$g: $bound> (&self, $($param: $param_type),*) -> $ret
        where $($where_clause)+
        {
            match self.0 {
                #[cfg(feature = "mysql")]
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "postgres")]
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                // DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "sqlite")]
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                // DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
                _ => {panic!()}
            }
        }
    };

    // 模式3: 单个泛型参数，没有 where 子句
    ($method:ident <$g:ident> ($($param:ident: $param_type:ty),*) -> $ret:ty) => {
        fn $method <$g> (&self, $($param: $param_type),*) -> $ret {
            match self.0 {
                #[cfg(feature = "mysql")]
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "postgres")]
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                // DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                #[cfg(feature = "sqlite")]
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                // DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
                _=>{panic!()}
            }
        }
    };
}

pub struct DbTypeWrapper(DbType);

impl From<DbType> for DbTypeWrapper {
    fn from(db_type: DbType) -> Self {
        DbTypeWrapper(db_type)
    }
}

impl WhereSqlGenerator for DbTypeWrapper {

}

impl QueryWrapperSqlGenerator for DbTypeWrapper{

}

impl PageSqlGenerator for DbTypeWrapper {
    impl_db_method_generic!(gen_page_query_sql(query_sql: &str, current_page: u64, page_size: u64) -> (String, u64, u64));
    impl_db_method_generic!(gen_page_total_sql(query_sql: &str) -> String);
}

impl BaseSqlGenerator for DbTypeWrapper {
    impl_db_method_generic!(gen_insert_and_get_id_sql<E>(e: &E) -> (String, Vec<ParamValue>)where E: Entity);
    impl_db_method_generic!(gen_insert_batch_sql<E>(e_vec: &Vec<E>) -> (String, Vec<ParamValue>)where E: Entity);
}


macro_rules! impl_executor_methods {
    // 匹配无泛型参数的方法
    ($self:ident, $method:ident($($arg:ident),*)) => {
        match $self.0 {
            #[cfg(feature = "mysql")]
            DbType::Mysql => MYSQL_SQL_EXECUTOR.$method($($arg),*).await,
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.$method($($arg),*).await,
            // #[cfg(feature = "oracle")]
            // DbType::Oracle => todo!(),
            #[cfg(feature = "postgres")]
            DbType::Postgres => POSTGRES_SQL_EXECUTOR.$method($($arg),*).await,
            // #[cfg(feature = "sqlserver")]
            // DbType::SqlServer => todo!(),
            _ => {panic!()},
        }
    };

    // 匹配有泛型参数的方法
    ($self:ident, $method:ident<$($gen:ident),*>($($arg:ident),*)) => {
        match $self.0 {
            #[cfg(feature = "mysql")]
            DbType::Mysql => MYSQL_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            // #[cfg(feature = "oracle")]
            // DbType::Oracle => ORACLE_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            #[cfg(feature = "postgres")]
            DbType::Postgres => POSTGRES_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            // #[cfg(feature = "sqlserver")]
            // DbType::SqlServer => todo!(),
            _ => {panic!()},
        }
    };
}

pub struct DbTypeOccupy;
impl RowType for DbTypeOccupy {
    fn col_to_v_by_index(&self, _: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        Err(DatabaseError::CommonError("DbTypeOccupy::col_to_v_by_index".to_string()))
    }

    fn col_to_v_by_name(&self, _: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        Err(DatabaseError::CommonError("DbTypeOccupy::col_to_v_by_name".to_string()))
    }
}


// 然后可以更简洁地实现
impl Executor for DbTypeWrapper {
    type Row<'a> = DbTypeOccupy;
    type Conn = DbTypeOccupy;

    async fn query<T, R, F, Q>(&self, _: Arc<std::sync::Mutex<Self::Conn>>, _: &str, _: &Vec<ParamValue>, _: F, _: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        Err(DatabaseError::CommonError("DbType::query not implemented".to_string()))?
    }

    async fn execute(&self, _: Arc<std::sync::Mutex<Self::Conn>>, _: &str, _: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        Err(DatabaseError::CommonError("DbType::execute not implemented".to_string()))?
    }

    fn get_conn_ref(&self) -> Result<Arc<std::sync::Mutex<Self::Conn>>, DatabaseError> {
        Err(DatabaseError::CommonError("DbType::get_conn_ref not implemented".to_string()))?
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        Err(DatabaseError::CommonError("DbType::get_conn not implemented".to_string()))?
    }

    async fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        impl_executor_methods!(self, query_some(sql, params))
    }

    async fn query_one<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        impl_executor_methods!(self, query_one(sql, params))
    }

    async fn query_count(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        impl_executor_methods!(self, query_count(sql, params))
    }

    async fn insert<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity
    {
        impl_executor_methods!(self, insert<E>(sql, params))
    }

    async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        impl_executor_methods!(self, insert_batch<E>(sql, params))
    }

    async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        impl_executor_methods!(self, delete(sql, params))
    }

    async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        impl_executor_methods!(self, update(sql, params))
    }
}


impl DbRegister for DbTypeWrapper {
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        match self.0 {
            #[cfg(feature = "mysql")]
            DbType::Mysql => MYSQL_DB_REGISTER.register_db(config),
            #[cfg(feature = "sqlite")]
            DbType::Sqlite => SQLITE_DB_REGISTER.register_db(config),
            // #[cfg(feature = "oracle")]
            // DbType::Oracle => ORACLE_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            #[cfg(feature = "postgres")]
            DbType::Postgres => POSTGRES_DB_REGISTER.register_db(config),
            // #[cfg(feature = "sqlserver")]
            // DbType::SqlServer => SQLSERVER_SQL_EXECUTOR.$method::<$($gen),*>($($arg),*).await,
            _ => {panic!()},
        }
    }
}

impl DbTypeWrapper{
    pub fn register_dbs(configs: Vec<DbConfig>) -> Result<(), DatabaseError>{
        for config in configs {
            DbTypeWrapper(config.db_type).register_db(&config)?;
        }
        Ok(())
    }
}