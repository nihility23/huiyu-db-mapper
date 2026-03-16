use crate::entities::UserEntity;
use crate::mappers::UserMapper;
use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;
use huiyu_db_util::huiyu_db_mapper_core::pool::datasource::DB_NAME_REGISTRY;

#[tokio::test]
async fn test_datasource() {
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                           Some("10.150.2.200".to_string()),
                                           Some(5432),
                                           Some("postgres".to_string()),
                                           Some("123456".to_string()),
                                           Some("postgres".to_string()),
                                           Some("fhds".to_string()),
                                           "default".to_string());
    let db_config_sqlite = DbConfig::new(
        DbType::Sqlite,
        None, None,None, None,
        Some("E:\\test\\huiyu.db".to_string()),  None,
        "test".to_string());
    DbTypeWrapper::register_dbs(vec![db_config_sqlite,db_config_postgres]).expect("Failed to register db");

    let user = query_user().await;
    println!("{:?}", user);
}
#[datasource("test")]
async fn query_user()->UserEntity{
    let res = {
        let res = UserMapper::select_by_key(&1i64).await.unwrap();
        res.unwrap()
    };
    res
}