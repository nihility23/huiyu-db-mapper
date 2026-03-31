use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

pub const MYSQL_SQL_GENERATOR:MysqlSqlGenerator = MysqlSqlGenerator{};
pub struct MysqlSqlGenerator;

impl PageSqlGenerator for MysqlSqlGenerator {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String,u64,u64) {
        (format!("select * from({}) limit ? , ?",query_sql),page_size, (current_page-1)*page_size)
    }
}

impl WhereSqlGenerator for MysqlSqlGenerator {

}

impl BaseSqlGenerator for MysqlSqlGenerator{
    fn gen_case_sensitive(&self, column:&str)->String{
        format!("`{}`",column)
    }

    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {

        let (sql,params) = self.gen_insert_one_sql(e);
        (format!("{};{}",sql,"SELECT LAST_INSERT_ID()"),params)
    }
}

impl QueryWrapperSqlGenerator for MysqlSqlGenerator {

}