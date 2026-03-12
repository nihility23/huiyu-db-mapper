use std::sync::Arc;
use tokio::sync::Mutex;

use huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::sql::executor::{Executor, RowType};
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};
#[cfg(feature = "postgres")]
use huiyu_db_mapper_postgres::postgres::postgres_sql_generator::POSTGRES_SQL_GENERATOR;
#[cfg(feature = "sqlite")]
use huiyu_db_mapper_sqlite::sqlite::sqlite_sql_generator::SQLITE_SQL_GENERATOR;
#[cfg(feature = "postgres")]
use huiyu_db_mapper_postgres::postgres::postgres_executor::POSTGRES_SQL_EXECUTOR;
#[cfg(feature = "sqlite")]
use huiyu_db_mapper_sqlite::sqlite::sqlite_executor::SQLITE_SQL_EXECUTOR;
#[cfg(feature = "mysql")]
use huiyu_db_mapper_mysql::mysql::mysql_sql_generator::MYSQL_SQL_GENERATOR;
#[cfg(feature = "mysql")]
use huiyu_db_mapper_mysql::mysql::mysql_executor::MYSQL_SQL_EXECUTOR;

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

pub struct QueryDbType(DbType);

impl From<DbType> for QueryDbType {
    fn from(db_type: DbType) -> Self {
        QueryDbType(db_type)
    }
}

impl WhereSqlGenerator for QueryDbType {

}


impl QueryWrapperSqlGenerator for QueryDbType{

}

impl PageSqlGenerator for QueryDbType {
    impl_db_method_generic!(gen_page_query_sql(query_sql: &str, current_page: u64, page_size: u64) -> (String, u64, u64));
    impl_db_method_generic!(gen_page_total_sql(query_sql: &str) -> String);
}

impl BaseSqlGenerator for QueryDbType {
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
            // DbType::Oracle => todo!(),
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
    fn col_to_v_by_index(&self, col_index: usize) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        Err(DatabaseError::CommonError("DbTypeOccupy::col_to_v_by_index".to_string()))
    }

    fn col_to_v_by_name(&self, col_name: &str) -> Result<ParamValue, DatabaseError>
    where
        Self: Sized
    {
        Err(DatabaseError::CommonError("DbTypeOccupy::col_to_v_by_name".to_string()))
    }
}

impl AsRef<DbTypeOccupy> for DbTypeOccupy {
    fn as_ref(&self) -> &DbTypeOccupy {
        todo!()
    }
}


// 然后可以更简洁地实现
impl Executor for QueryDbType {
    type Row<'a> = DbTypeOccupy;
    type Conn = DbTypeOccupy;
    // type ConnWrapper = DbTypeOccupy;

    async fn query<T, R, F, Q>(&self, conn: Arc<std::sync::Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        todo!()
    }

    async fn execute(&self, conn: Arc<std::sync::Mutex<Self::Conn>>, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        todo!()
    }



    async fn query_basic<T, R, F, Q>(&self, sql: &str, params: &Vec<ParamValue>, mapper: F, processor: Q) -> Result<R, DatabaseError>
    where
        T: Send + 'static,
        R: Send + 'static,
        F: for<'a> Fn(&Self::Row<'a>) -> Result<T, DatabaseError> + Send + 'static,
        Q: FnOnce(Vec<T>) -> Result<R, DatabaseError> + Send + 'static
    {
        //impl_executor_methods!(self, query_basic(sql, params,mapper, processor))
        Err(DatabaseError::CommonError(format!("DbType::query_basic")))
    }

    fn row_to_e<'a, E>(row: &Self::Row<'a>) -> Result<E, DatabaseError>
    where
        E: Entity
    {
        todo!()
    }

    fn get_conn_ref(&self) -> Result<Arc<std::sync::Mutex<Self::Conn>>, DatabaseError> {
        todo!()
    }

    async fn get_conn(&self) -> Result<Self::Conn,DatabaseError> {
        todo!()
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
