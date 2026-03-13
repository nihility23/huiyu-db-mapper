use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;

#[tokio::test]
async fn test(){
    let db_config_mysql = DbConfig::new(DbType::Mysql,
                                        Some("10.150.6.6".to_string()),
                                        Some(3306),
                                        Some("root".to_string()),
                                        Some("1qaz!QAZ".to_string()),
                                        Some("test".to_string()),
                                        Some("test".to_string()),
                                        "mysql".to_string());
    DbTypeWrapper::register_dbs(vec![db_config_mysql]).expect("Failed to register db");
}