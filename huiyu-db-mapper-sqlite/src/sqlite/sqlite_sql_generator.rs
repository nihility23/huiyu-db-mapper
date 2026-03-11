use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

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
        let (insert_sql, params) = self.gen_insert_one_sql(e);
        let sql = format!("{} {};", insert_sql, " RETURNING id".to_string());
        (sql, params)
    }
}

impl QueryWrapperSqlGenerator for SqliteSqlGenerator {}