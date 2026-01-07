use crate::base::entity::Entity;
use crate::base::param::ParamValue;
use crate::sql::generator::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

pub const ORACLE_SQL_GENERATOR:OracleSqlGenerator=OracleSqlGenerator{};
pub struct OracleSqlGenerator;

impl WhereSqlGenerator for OracleSqlGenerator {

}

impl PageSqlGenerator for OracleSqlGenerator {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String, u64, u64) {
        (format!("SELECT * FROM ( SELECT t.*, ROWNUM rnum  FROM (  {}  ) t  WHERE ROWNUM <= ? ) WHERE rnum > ? ",query_sql),(current_page-1)*page_size,current_page*page_size)
    }
}

impl BaseSqlGenerator for OracleSqlGenerator {
    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {
        todo!()
    }
}

impl QueryWrapperSqlGenerator for OracleSqlGenerator {}