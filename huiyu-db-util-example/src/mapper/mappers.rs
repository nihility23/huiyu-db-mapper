use std::alloc;
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
        
        // 支持多个 OccupyQueryMapper 的示例
        #[select("select * from t_role where 1=1 and #{query_wrapper} and #{query_wrapper}")]
        async fn query_role_by_multiple_wrappers<'a>(wrapper1: &OccupyQueryMapper<'a>, wrapper2: &OccupyQueryMapper<'a>) -> Result<Vec<RoleDTO>, DatabaseError>;
    }
    
    execute_impl!{
        #[sql("update t_role set role_code = ? where id = ?")]
        async fn update_role_code(id: i64, role_code: String) -> Result<u64, DatabaseError>;
        #[sql("create table t_test(id: int)")]
        async fn create_table_test(id: i64) -> Result<u64, DatabaseError>;
        #[sql("CREATE TABLE Employees_?# (
                EmployeeID INTEGER PRIMARY KEY,
                Name TEXT NOT NULL,
                Age INTEGER
            );
        ")]
        async fn create_table_employees(idx: i64) -> Result<u64, DatabaseError>;
    }
    // pub async fn create_table_employees(idx: i64) -> Result<u64, DatabaseError> {
    //     let mut sql = "CREATE TABLE Employees_? (
    //                 EmployeeID INTEGER PRIMARY KEY,
    //                 Name TEXT NOT NULL,
    //                 Age INTEGER
    //             );
    //         ".to_string();
    //     let mut param_vec:Vec<ParamValue> = <[_]>::into_vec(
    //         ::alloc::boxed::box_new([(idx.into())])
    //     );
    //     while sql.contains("?#") {
    //         sql = sql.replace("?#", &param_vec[0].to_string().as_str());
    //         param_vec.remove(0);
    //     }
    //
    //     Self::exec::<_, _, _, _, u64>(|db_type: DbType| {
    //         (db_type, sql, param_vec)
    //     }, async |(db_type, sql, param_vec)| {
    //         <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &param_vec).await
    //     }).await
    // }
    // pub async fn update_role_code(id: i64, role_code: String) -> Result<u64, DatabaseError> {
    //     let mut sql = "update t_role set role_code = ? where id = ?".to_string();
    //     let mut param_vec: Vec<ParamValue> = vec![id.into(), role_code.into()];
    //     while sql.contains("?#") {
    //         sql = sql.replace("?#", param_vec[0].to_string().as_str());
    //         param_vec.remove(0);
    //     }
    //
    //     Self::exec::<_, _, _, _, u64>(|db_type: DbType| {
    //         (db_type, sql, param_vec)
    //     }, async |(db_type, sql, param_vec)| {
    //         <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &param_vec).await
    //     }).await
    // }
    // pub async fn create_table_test(id: i64) -> Result<u64, DatabaseError> {
    //     let mut sql = "create table t_test(id: int)".to_string();
    //     let mut param_vec = vec![id.into()];
    //     while sql.contains("?#") {
    //             sql = sql.replace("?#", &param_vec[0].to_string());
    //         param_vec.remove(0);
    //     }
    //
    //     Self::exec::<_, _, _, _, u64>(|db_type: DbType| {
    //         (db_type, sql, param_vec)
    //     }, async |(db_type, sql, param_vec)| {
    //         <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &param_vec).await
    //     }).await
    // }
    // pub async fn create_table_employees(idx: i64) -> Result<u64, DatabaseError> {
    //     let mut sql = "CREATE TABLE Employees_? (
    //                 EmployeeID INTEGER PRIMARY KEY,
    //                 Name TEXT NOT NULL,
    //                 Age INTEGER
    //             );
    //         ".to_string();
    //     let mut param_vec = <[_]>::into_vec(
    //         ::alloc::boxed::box_new([(idx.into())])
    //     );
    //     while sql.contains("?#") {
    //         sql = sql.replace("?#", &param_vec[0].to_string());
    //         param_vec.remove(0);
    //     }
    //
    //     Self::exec::<_, _, _, _, u64>(|db_type: DbType| {
    //         (db_type, sql, param_vec)
    //     }, async |(db_type, sql, param_vec)| {
    //         <DbType as Into<DbTypeWrapper>>::into(db_type).execute_sql(sql.as_str(), &param_vec).await
    //     }).await
    // }
}

pub struct PermissionMapper;

impl BaseMapper<PermissionEntity> for PermissionMapper {

}