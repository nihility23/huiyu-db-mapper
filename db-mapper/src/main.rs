mod sql;
mod query;

mod base;

use crate::base::db_type::DbType;
use crate::base::entity::Entity;
use crate::base::page::Page;
use crate::base::param::ParamValue;
use crate::query::query_wrapper::QueryWrapper;
use crate::sql::generator::sql_generator::QueryWrapperSqlGenerator;
use db_macros::CheckAllOption;

#[derive(CheckAllOption)]
struct User{
    name:Option<String>,
    age:Option<u16>
}


fn main() {
    
}
