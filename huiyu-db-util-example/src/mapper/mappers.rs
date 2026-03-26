use huiyu_db_util::huiyu_db_mapper_core::sql::sql_generator::QueryWrapperSqlGenerator;
use crate::entity::entities::{PermissionEntity, RoleEntity, UserEntity, UserRoleEntity};
use crate::entity::mappings::RoleDTO;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper::select_impl;
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

// select_impl! {
//
//
//         pub trait RolePermissionMapper:BaseMapper<RolePermissionEntity> {
//             #[select("select *from t_user where id = ?")]
//             async fn query_role_dtos(id:String)->Result<Vec<RoleDTO>, DatabaseError>;
//
//             #[select("select *from t_user where id = ?")]
//             async fn query_role_page(page:Page,name:String)->Result<PageRes<RoleDTO>, DatabaseError>;
//
//             #[select("select *from t_user where role_code = ? and status = ?")]
//             async fn query_role_first(name:String, status: i8)->Result<Option<RoleDTO>, DatabaseError>;
//
//             #[select("select role_name from t_user where role_code = ? and status = ?")]
//             async fn query_role_name(name:String, status: i8)->Result<Option<String>, DatabaseError>;
//         }
//     }

impl RoleMapper {
    select_impl! {

        #[select("select * from t_role where id = ?")]
        async fn query_role_dtos(id: String) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_role where id = ?")]
        async fn query_role_page(page: Page, name: String) -> Result<PageRes<RoleDTO>, DatabaseError>;

        #[select("select * from t_role where role_code = ? and status = ?")]
        async fn query_role_first(name: String, status: i8) -> Result<Option<RoleDTO>, DatabaseError>;

        #[select("select role_name from t_role where role_code = ? and status = ?")]
        #[value]   // 标记为简单值类型
        async fn query_role_name(name: String, status: i8) -> Result<Option<String>, DatabaseError>;

        #[select("select * from t_role where 1=1 and #{query_wrapper}")]
        async fn query_role_dtos_by_query_wrapper<'a>(query_wrapper: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;

        #[select("select * from t_role  where 1=1 and #{query_wrapper}")]
        async fn query_role_page_query_wrapper<'a>(page: Page, query_wrapper: &OccupyQueryMapper<'a>) -> Result<PageRes<RoleDTO>, DatabaseError>;

        #[select("select * from t_role where 1=1 and #{query_wrapper}")]
        async fn query_role_first_query_wrapper<'a>(query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<RoleDTO>, DatabaseError>;

        #[select("select role_name from t_user u left join t_user_role ur on ur.user_id = u.id left join t_role r on r.id = ur.role_id where 1=1 and #{query_wrapper}")]
        #[value]   // 标记为简单值类型
        async fn query_role_name_query_wrapper<'a>(query_wrapper: &OccupyQueryMapper<'a>) -> Result<Option<String>, DatabaseError>;
    }
}

pub struct PermissionMapper;

impl BaseMapper<PermissionEntity> for PermissionMapper {

}