use huiyu_db_util::huiyu_db_mapper::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_util::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_util::huiyu_db_mapper_core::base::db_type::DbType;

pub fn init_dbs(){
    init_postgres();
    init_mysql();
    init_sqlite();
    init_oracle();
}

fn init_postgres(){
    println!("init postgres");
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                           "postgres".to_string(),
                                           Some("10.150.2.200".to_string()),
                                           Some(5432),
                                           Some("postgres".to_string()),
                                           Some("123456".to_string()),
                                           Some("postgres".to_string()),
                                           Some("fhds".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_postgres]).expect("Failed to register db");
}
fn init_mysql(){
    println!("init mysql");
    let db_config_mysql = DbConfig::new(DbType::Mysql,
                                        "mysql".to_string(),
                                        Some("10.150.6.6".to_string()),
                                        Some(3306),
                                        Some("root".to_string()),
                                        Some("1qaz!QAZ".to_string()),
                                        Some("test".to_string()),
                                        Some("test".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_mysql]).expect("Failed to register db");
}
fn init_sqlite(){
    println!("init sqlite");
    let db_config_sqlite = DbConfig::new(
        DbType::Sqlite,
        "sqlite".to_string(),
        None, None, None, None,
        Some("E:\\test\\huiyu.db".to_string()), None
    );
    DbTypeWrapper::register_dbs(vec![db_config_sqlite]).expect("Failed to register db");
}

fn init_oracle(){
    println!("init oracle");
    let db_config_mysql = DbConfig::new(DbType::O,
                                        "mysql".to_string(),
                                        Some("10.150.6.6".to_string()),
                                        Some(3306),
                                        Some("root".to_string()),
                                        Some("1qaz!QAZ".to_string()),
                                        Some("test".to_string()),
                                        Some("test".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_mysql]).expect("Failed to register db");
}
