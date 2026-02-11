use crate::base::entity::Entity;
use crate::base::param::ParamValue;
use crate::sql::sql_generator::{
    BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator,
};

pub const MYSQL_SQL_GENERATOR: MysqlSqlGenerator = MysqlSqlGenerator {};

pub struct MysqlSqlGenerator;

impl WhereSqlGenerator for MysqlSqlGenerator {}

impl PageSqlGenerator for MysqlSqlGenerator {
    fn gen_page_query_sql(
        &self,
        query_sql: &str,
        current_page: u64,
        page_size: u64,
    ) -> (String, u64, u64) {
        (
            format!("select * from({}) limit ?, ?", query_sql),
            (current_page - 1) * page_size,
            page_size,
        )
    }
}

impl BaseSqlGenerator for MysqlSqlGenerator {
    fn gen_insert_and_get_id_sql<E>(&self, e: &E) -> (String, Vec<ParamValue>)
    where
        E: Entity,
    {
        let (sql, params) = self.gen_insert_one_sql::<E>(e);
        (format!("{}; {};", sql, "SELECT LAST_INSERT_ID()"), params)
    }
}

impl QueryWrapperSqlGenerator for MysqlSqlGenerator {}
