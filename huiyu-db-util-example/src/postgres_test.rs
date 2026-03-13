use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;

#[tokio::test]
async fn test(){
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                           Some("10.150.2.200".to_string()),
                                           Some(5432),
                                           Some("postgres".to_string()),
                                           Some("123456".to_string()),
                                           Some("postgres".to_string()),
                                           Some("fhds".to_string()),
                                           "default".to_string());
    DbTypeWrapper::register_dbs(vec![db_config_postgres]).expect("Failed to register db");
}