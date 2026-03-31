use std::alloc;
use chrono::{DateTime, Local};
use huiyu_db_util::huiyu_db_macros::mapper;
use huiyu_db_util::huiyu_db_mapper_core::sql::sql_generator::QueryWrapperSqlGenerator;
use crate::entity::entities::{PermissionEntity, RoleEntity, UserEntity, UserRoleEntity};
use crate::entity::mappings::RoleDTO;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper::{execute_impl, select_impl};
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_util::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_util::huiyu_db_mapper_core::base::page::{Page, PageRes};
use huiyu_db_util::huiyu_db_mapper_core::base::param::ParamValue;
use huiyu_db_util::huiyu_db_mapper_core::sql::executor::Executor;
use huiyu_db_util::huiyu_db_mapper_core::sql::sql_generator::PageSqlGenerator;
use huiyu_db_util::huiyu_db_mapper::query::query_wrapper_occupy::OccupyQueryMapper;

pub struct UserMapper;

impl UserMapper {

    select_impl!{
        #[select("select username from t_user where id = ?")]
        #[value]
        async fn select_name_by_id(id:i64)->Result<Option<String>,DatabaseError>;

        #[select("select id,username from t_user where username like concat('%',?,'%')")]
        async fn select_name_by_page(page:Page,name:String)->Result<PageRes<RoleDTO>,DatabaseError>;
    }
}

impl BaseMapper<UserEntity> for UserMapper {}

pub struct UserRoleMapper;

impl BaseMapper<UserRoleEntity> for UserRoleMapper {}

pub struct RoleMapper;

impl BaseMapper<RoleEntity> for RoleMapper {}

impl RoleMapper {
    select_impl! {

        #[select("select id,role_name,create_time from t_role where id = ?")]
        async fn query_role_dtos(id: String) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_role where id like '%'||?#||'%'")]
        async fn query_role_page(page: Page, name: String) -> Result<PageRes<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role where role_code = ? and status = ?")]
        async fn query_role_first(name: String, status: i8) -> Result<Option<RoleDTO>, DatabaseError>;
        
        #[select("select role_name from t_role where role_code = ? and status = ?")]
        #[value]   // 标记为简单值类型
        async fn query_role_name(name: String, status: i8) -> Result<Option<String>, DatabaseError>;
        
        #[select("select * from t_role where role_code = ? and #{qw}")]
        async fn query_role_dtos_by_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role  where 1=1 and #{qw}")]
        async fn query_role_page_query_wrapper<'a>(page: Page,name:String,name1:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<PageRes<RoleDTO>, DatabaseError>;
        
        #[select("select * from t_role where role_name like concat('%',?#,'%') and #{qw}")]
        async fn query_role_first_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<RoleDTO>, DatabaseError>;
        
        #[select("select role_name from t_user u left join t_user_role ur on ur.user_id = u.id left join t_role r on r.id = ur.role_id where role_name like concat('%',?#,'%') and #{qw}")]
        #[value]   // 标记为简单值类型
        async fn query_role_name_query_wrapper<'a>(name:String,query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<String>, DatabaseError>;
        
        // 支持多个 OccupyQueryMapper 的示例
        #[select("select * from t_role where 1=1 and role_code =?# and #{qw} and #{qw}")]
        async fn query_role_by_multiple_wrappers<'a>(code:String,wrapper1: &OccupyQueryMapper<'a>, wrapper2: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
    }
    
    execute_impl!{
        #[sql("update t_role set update_time = ? where #{qw} and #{qw}")]
        async fn update_role(update_time: DateTime<Local>, query_wrapper: &OccupyQueryMapper<'_>,query_wrapper1: &OccupyQueryMapper<'_>) -> Result<u64, DatabaseError>;
        #[sql("create table t_test(id: int)")]
        async fn create_table_test(id: i64) -> Result<u64, DatabaseError>;
        #[sql("CREATE TABLE Employees_?@ (
                EmployeeID INTEGER PRIMARY KEY,
                Name TEXT NOT NULL,
                Age INTEGER
            );
        ")]
        async fn create_table_employees(idx:i64) -> Result<u64, DatabaseError>;
    }
}

#[mapper(PermissionEntity)]
pub struct PermissionMapper;

// impl BaseMapper<PermissionEntity> for PermissionMapper {
// 
// }