use crate::base::entity::Entity;
use crate::base::param::ParamValue;
use crate::sql::generator::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

pub const SQLITE_SQL_GENERATOR:SqliteSqlGenerator= SqliteSqlGenerator {};
pub struct SqliteSqlGenerator;

impl PageSqlGenerator for SqliteSqlGenerator {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String,u64,u64) {
        (format!("select * from({}) limit ? offset ?",query_sql),page_size, (current_page-1)*page_size)
    }
}

impl WhereSqlGenerator for SqliteSqlGenerator {

}

impl BaseSqlGenerator for SqliteSqlGenerator{
    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {
        todo!()
    }
}

impl QueryWrapperSqlGenerator for SqliteSqlGenerator {}