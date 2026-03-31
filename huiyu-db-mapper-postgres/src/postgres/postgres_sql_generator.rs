use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

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
        let (sql,params) = self.gen_insert_one_sql(e);
        (format!("{} returning {}", sql, E::key_name()), params)
    }
    
}

impl QueryWrapperSqlGenerator for PostgresSqlGenerator {
    fn gen_case_sensitive(&self, column:&str)->String{
        format!("\"{}\"",column)
    }
}