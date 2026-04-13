use crate::common::db::init_dbs;
use crate::entity::entities::{UserEntity, UserRoleEntity};
use crate::mapper::mappers::{RoleMapper, UserMapper, UserRoleMapper};
use chrono::{Local, NaiveDateTime, TimeZone};
use huiyu_db_mapper::huiyu_db_mapper_macros::{datasource, transactional};
use huiyu_db_mapper::huiyu_db_mapper_impl::query::base_mapper::BaseMapper;
use huiyu_db_mapper::huiyu_db_mapper_impl::query::transactional::transactional_exec;
use huiyu_db_mapper::huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper::huiyu_db_mapper_core::base::mapping::Mapping;

#[tokio::test]
pub async fn test()-> Result<(), DatabaseError>{
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    init_dbs();
    // insert_one().await;
    test_transaction().await?;
    Ok(())
}

#[datasource("sqlite")]
async fn test_transaction()-> Result<(), DatabaseError>{
    let mut user_role1 = UserRoleEntity::new();
    user_role1.create_time = Some(chrono::Local::now());
    user_role1.user_id = Some(12);
    user_role1.role_id = Some(String::from("role_003"));

    let mut user_role2 = UserRoleEntity::new();
    user_role2.create_time = Some(chrono::Local::now());
    user_role2.user_id = Some(131111111);
    user_role2.role_id = Some(String::from("role_002"));

    let res = RoleMapper::delete_by_key(&"1 or 101".to_string()).await;
    println!("{:?}", res);

    let res = transactional_exec(async ||{
        let res = UserRoleMapper::insert(&mut user_role1).await?;
        let res2 = UserRoleMapper::insert(&mut user_role2).await?;
        Ok(())
    }).await?;

    let res = transactional!({
        UserRoleMapper::insert(&mut user_role1).await?;
        UserRoleMapper::insert(&mut user_role2).await?;
        Ok(())
    })?;
    Ok(())
}
async fn insert_one(){
    let mut user = UserEntity::new();
    user.create_time = Some(chrono::Local::now());
    user.username = Some(String::from(format!("test_{}", chrono::Local::now().timestamp())));
    user.password = Some(String::from("123456"));
    user.height = Some(175.0);
    user.email = Some(String::from("test@example.com"));
    let naive = NaiveDateTime::parse_from_str("2000-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    user.birthday = Some(Local.from_local_datetime(&naive).unwrap());
    user.gender = Some(1);
    user.is_active = Some(true);
    user.phone = Some(String::from("12345678901"));
    user.profile = Some(String::from("This is a test profile."));
    user.real_name = Some(String::from("Test User"));
    let res = UserMapper::insert(&mut user).await.expect("Failed to insert user");
    println!("Insert result: {}", res.unwrap());



}


