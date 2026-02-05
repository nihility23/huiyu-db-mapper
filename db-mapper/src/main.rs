mod sql;
mod query;

mod base;
mod db;
use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::page::Page;
use crate::base::param::ParamValue;
use crate::query::query_wrapper::QueryWrapper;
use db_mapper::sql::sql_generator::QueryWrapperSqlGenerator;
use db_macros::CheckAllOption;

#[derive(CheckAllOption)]
struct User{
    name:Option<String>,
    age:Option<u16>
}


fn main() {
    
}
