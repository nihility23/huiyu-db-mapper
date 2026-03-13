use chrono::{ Local, NaiveDateTime, TimeZone};
use rustlog::{set_level, set_target, Level, Target};
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper::query::db_type::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_util::huiyu_db_mapper_core::base::entity::Entity;
use crate::entities::UserEntity;
use crate::mappers::UserMapper;

#[tokio::test]
pub async fn test(){
    init();
    insert_one().await;
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


fn init(){
    set_target(Target::Stderr);
    set_level(Level::Info);
    let db_config_sqlite = DbConfig::new(
        DbType::Sqlite,
        None, None,None, None,
        Some("E:\\test\\huiyu.db".to_string()),  None,
        "default".to_string());
    DbTypeWrapper::register_dbs(vec![db_config_sqlite]).expect("Failed to register db");

}