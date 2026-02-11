use db_mapper::sql::sql_generator::QueryWrapperSqlGenerator;
mod mapper_test;
mod user_mapper;

use crate::mapper_test::UserEntity;
use db_mapper::base::db_type::DbType;
use db_mapper::base::page::Page;
use db_mapper::base::param::ParamValue;
use db_mapper::query::query_wrapper::QueryWrapper;

fn main() {
    let mut query_wrapper = QueryWrapper::<UserEntity>::new()
        .ge("age",ParamValue::I16(18))
        .null("sex")
        .like_left("name",ParamValue::String("zhangsan".to_string()))
        .not_null("name");


    let (sql,params) = DbType::Sqlite.gen_query_sql(&mut query_wrapper);
    println!("Sqlite sql: {}",sql);
    println!("Sqlite params: {:?}",params);

    let (page_sql,total_sql,params) = DbType::Sqlite.gen_page_sql(&Page{current_page:1,page_size:10},&query_wrapper);
    println!("Sqlite page sql: {}",page_sql);
    println!("Sqlite total sql: {}",total_sql);
    println!("Sqlite params: {:?}",params);

    let (page_sql,total_sql,params) = DbType::Mysql.gen_page_sql(&Page{current_page:1,page_size:10},&query_wrapper);
    println!("Mysql page sql: {}",page_sql);
    println!("Mysql total sql: {}",total_sql);
    println!("Mysql params: {:?}",params);

    let (page_sql,total_sql,params) = DbType::Oracle.gen_page_sql(&Page{current_page:1,page_size:10},&query_wrapper);
    println!("Oracle page sql: {}",page_sql);
    println!("Oracle total sql: {}",total_sql);
    println!("Oracle params: {:?}",params);
}
