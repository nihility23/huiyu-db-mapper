
#[cfg(test)]
mod tests {
    use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper;
    use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
    use huiyu_db_util::huiyu_db_mapper::query::query_wrapper_occupy::OccupyQueryMapper;
    use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;
    use huiyu_db_util::huiyu_db_mapper_core::sql::sql_generator::QueryWrapperSqlGenerator;
    use crate::mapper::mappers::RoleMapper;

    #[test]
    fn test(){
        let db_type_wrapper = DbTypeWrapper::from(DbType::Sqlite);
        println!("{:?}", db_type_wrapper);

        let occupy_query_wrapper = OccupyQueryMapper::new().eq("t.a",23).lt("t.b", 23);
        let sql = db_type_wrapper.gen_where_sql(&occupy_query_wrapper);
        println!("{}", sql.unwrap().0);

    }

    #[test]
    fn test_multiple_wrappers() {

        
    }
}