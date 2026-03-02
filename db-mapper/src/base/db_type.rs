use crate::base::entity::Entity;
use crate::base::error::DatabaseError;
use crate::base::param::ParamValue;
use crate::db::mysql::mysql_executor::MYSQL_SQL_EXECUTOR;
use crate::db::mysql::mysql_sql_generator::MYSQL_SQL_GENERATOR;
use crate::db::oracle::oracle_sql_generator::ORACLE_SQL_GENERATOR;
use crate::db::postgres::postgres_sql_generator::POSTGRES_SQL_GENERATOR;
use crate::db::sqlite::sqlite_executor::SQLITE_SQL_EXECUTOR;
use crate::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};
use crate::db::sqlite::sqlite_sql_generator::SQLITE_SQL_GENERATOR;
use crate::db::sqlserver::sqlserver_sql_generator::SQL_SERVER_SQL_GENERATOR;
use crate::sql::executor::Executor;

#[derive(Debug,Clone,Copy)]
pub enum DbType{
    Mysql,
    Sqlite,
    Oracle,
    Postgres,
    SqlServer,
}

macro_rules! impl_db_method_generic {
    ($method:ident($($param:ident: $param_type:ty),*) -> $ret:ty) => {
        fn $method(&self, $($param: $param_type),*) -> $ret {
            match self {
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
            }
        }
    };
    // 模式1: 单个泛型参数，有 where 子句
    ($method:ident <$g:ident> ($($param:ident: $param_type:ty),*) -> $ret:ty where $($where_clause:tt)+) => {
        fn $method <$g> (&self, $($param: $param_type),*) -> $ret
        where $($where_clause)+
        {
            match self {
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
            }
        }
    };

    // 模式2: 单个泛型参数带约束，有 where 子句
    ($method:ident <$g:ident: $bound:path> ($($param:ident: $param_type:ty),*) -> $ret:ty where $($where_clause:tt)+) => {
        fn $method <$g: $bound> (&self, $($param: $param_type),*) -> $ret
        where $($where_clause)+
        {
            match self {
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
            }
        }
    };

    // 模式3: 单个泛型参数，没有 where 子句
    ($method:ident <$g:ident> ($($param:ident: $param_type:ty),*) -> $ret:ty) => {
        fn $method <$g> (&self, $($param: $param_type),*) -> $ret {
            match self {
                DbType::Mysql => MYSQL_SQL_GENERATOR.$method($($param),*),
                DbType::Postgres => POSTGRES_SQL_GENERATOR.$method($($param),*),
                DbType::Oracle => ORACLE_SQL_GENERATOR.$method($($param),*),
                DbType::Sqlite => SQLITE_SQL_GENERATOR.$method($($param),*),
                DbType::SqlServer => SQL_SERVER_SQL_GENERATOR.$method($($param),*),
            }
        }
    };
}

impl WhereSqlGenerator for DbType {

}


impl QueryWrapperSqlGenerator for DbType{

}

impl PageSqlGenerator for DbType {
    impl_db_method_generic!(gen_page_query_sql(query_sql: &str, current_page: u64, page_size: u64) -> (String, u64, u64));
    impl_db_method_generic!(gen_page_total_sql(query_sql: &str) -> String);
}

impl BaseSqlGenerator for DbType {
    impl_db_method_generic!(gen_insert_and_get_id_sql<E>(e: &E) -> (String, Vec<ParamValue>)where E: Entity);
    impl_db_method_generic!(gen_insert_batch_sql<E>(e_vec: &Vec<E>) -> (String, Vec<ParamValue>)where E: Entity);
}

impl Executor for DbType {

    async fn query_some<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Vec<E>, DatabaseError>
    where
        E: Entity
    {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.query_some(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.query_some(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn query_one<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E>, DatabaseError>
    where
        E: Entity
    {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.query_one(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.query_one(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn query_count(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.query_count(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.query_count(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn insert<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<Option<E::K>, DatabaseError>
    where
        E: Entity
    {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.insert::<E>(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.insert::<E>(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn insert_batch<E>(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError>
    where
        E: Entity
    {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.insert_batch::<E>(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.insert_batch::<E>(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn delete(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.delete(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.delete(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }       
    }

    async fn update(&self, sql: &str, params: &Vec<ParamValue>) -> Result<u64, DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.update(sql, params).await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.update(sql, params).await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn start_transaction(&self) -> Result<(), DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.start_transaction().await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.start_transaction().await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn commit(&self) -> Result<(), DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.commit().await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.commit().await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }

    async fn rollback(&self) -> Result<(), DatabaseError> {
        match self { 
            DbType::Mysql => MYSQL_SQL_EXECUTOR.rollback().await,
            DbType::Sqlite => SQLITE_SQL_EXECUTOR.rollback().await,
            DbType::Oracle => todo!(),
            DbType::Postgres => todo!(),
            DbType::SqlServer => todo!(),
        }
    }
}