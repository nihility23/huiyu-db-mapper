mod sql;
mod query;

mod base;
mod db;
mod pool;
use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::page::Page;
use crate::base::param::ParamValue;
use crate::query::query_wrapper::QueryWrapper;
use db_mapper::sql::sql_generator::QueryWrapperSqlGenerator;

struct User{
    name:Option<String>,
    age:Option<u16>
}


fn main() {
    
}
