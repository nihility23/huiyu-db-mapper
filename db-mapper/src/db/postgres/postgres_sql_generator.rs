use crate::base::entity::Entity;
use crate::base::param::ParamValue;
use crate::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

pub const POSTGRES_SQL_GENERATOR:PostgresSqlGenerator = PostgresSqlGenerator{};
pub struct PostgresSqlGenerator;

impl PageSqlGenerator for PostgresSqlGenerator {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String,u64,u64) {
        (format!("select * from({}) limit ? offset ?",query_sql),page_size, (current_page-1)*page_size)
    }
}

impl WhereSqlGenerator for PostgresSqlGenerator {

}

impl BaseSqlGenerator for PostgresSqlGenerator{
    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {
        todo!()
    }
}

impl QueryWrapperSqlGenerator for PostgresSqlGenerator {}