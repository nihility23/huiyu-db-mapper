use crate::base::entity::Entity;
use crate::base::param::ParamValue;
use crate::sql::generator::mysql_sql_generator::MYSQL_SQL_GENERATOR;
use crate::sql::generator::oracle_sql_generator::ORACLE_SQL_GENERATOR;
use crate::sql::generator::postgres_sql_generator::POSTGRES_SQL_GENERATOR;
use crate::sql::generator::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};
use crate::sql::generator::sqlite_sql_generator::SQLITE_SQL_GENERATOR;
use crate::sql::generator::sqlserver_sql_generator::SQL_SERVER_SQL_GENERATOR;

#[derive(Debug,Clone,Copy)]
pub enum DbType{
    Mysql,
    Sqlite,
    Oracle,
    Postgres,
    SqlServer,
}

impl PageSqlGenerator for DbType {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String, u64, u64) {
        match self {
            DbType::Mysql=>{
                MYSQL_SQL_GENERATOR.gen_page_query_sql(query_sql, current_page, page_size)
            }
            DbType::Postgres=>{
                POSTGRES_SQL_GENERATOR.gen_page_query_sql(query_sql, current_page, page_size)
            }
            DbType::Oracle=>{
                ORACLE_SQL_GENERATOR.gen_page_query_sql(query_sql, current_page, page_size)
            }
            DbType::Sqlite=>{
                SQLITE_SQL_GENERATOR.gen_page_query_sql(query_sql, current_page, page_size)
            }
            DbType::SqlServer=>{
                SQL_SERVER_SQL_GENERATOR.gen_page_query_sql(query_sql, current_page, page_size)
            }
        }
    }
}

impl WhereSqlGenerator for DbType {

}

impl BaseSqlGenerator for DbType{
    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {
        match self {
            DbType::Mysql=>{
                MYSQL_SQL_GENERATOR.gen_insert_and_get_id_sql::<E>(e)
            }
            DbType::Postgres=>{
                POSTGRES_SQL_GENERATOR.gen_insert_and_get_id_sql::<E>(e)
            }
            DbType::Oracle=>{
                ORACLE_SQL_GENERATOR.gen_insert_and_get_id_sql::<E>(e)
            }
            DbType::Sqlite=>{
                SQLITE_SQL_GENERATOR.gen_insert_and_get_id_sql::<E>(e)
            }
            DbType::SqlServer=>{
                SQL_SERVER_SQL_GENERATOR.gen_insert_and_get_id_sql::<E>(e)
            }
        }
    }
}

impl QueryWrapperSqlGenerator for DbType{

}