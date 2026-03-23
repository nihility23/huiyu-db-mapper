use huiyu_db_mapper_core::base::entity::Entity;
use huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_mapper_core::sql::sql_generator::{BaseSqlGenerator, PageSqlGenerator, QueryWrapperSqlGenerator, WhereSqlGenerator};

pub const ORACLE_SQL_GENERATOR:OracleSqlGenerator = OracleSqlGenerator{};
pub struct OracleSqlGenerator;

impl PageSqlGenerator for OracleSqlGenerator {
    fn gen_page_query_sql(&self, query_sql: &str, current_page: u64, page_size: u64) -> (String,u64,u64) {
        // -- 查询第2页数据（每页10条）
        // SELECT *
        //     FROM (
        //         SELECT a.*, ROWNUM rnum
        //         FROM (
        //             SELECT * FROM employees
        //             ORDER BY hire_date DESC  -- 必须在内层排序
        //         ) a
        //         WHERE ROWNUM <= 20  -- 结束行=页码*每页条数
        //     )
        // WHERE rnum > 10;  -- 起始行=(页码-1)*每页条数

        let start = (current_page - 1) * page_size;
        let end = current_page * page_size;
        (format!("select * from(select t.*,ROWNUM rnum from ({}) t where ROWNUM <= ?) where rnum > ?",query_sql),end, start)
    }
}

impl WhereSqlGenerator for OracleSqlGenerator {

}

impl BaseSqlGenerator for OracleSqlGenerator{
    fn gen_insert_and_get_id_sql<E>(&self, e:&E) -> (String, Vec<ParamValue>)
    where
        E: Entity
    {
        // 不支持，通过uuid
        self.gen_insert_one_sql(e)
    }
}

impl QueryWrapperSqlGenerator for OracleSqlGenerator {}