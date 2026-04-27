use huiyu_db_mapper::huiyu_db_mapper_impl::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_mapper::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper::huiyu_db_mapper_core::base::db_type::DbType;

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
                                           Some("huiyu".to_string()),
                                           Some("public".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_postgres]).expect("Failed to register db");
}
fn init_mysql(){
    println!("init mysql");
    let db_config_mysql = DbConfig::new(DbType::Mysql,
                                        "mysql".to_string(),
                                        Some("10.150.6.7".to_string()),
                                        Some(3306),
                                        Some("root".to_string()),
                                        Some("123456".to_string()),
                                        Some("huiyu".to_string()),
                                        Some("".to_string()),
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
    let db_config_oracle = DbConfig::new(DbType::Oracle,
                                        "oracle".to_string(),
                                        Some("10.150.6.7".to_string()),
                                        Some(1521),
                                        Some("huiyu".to_string()),
                                        Some("123456".to_string()),
                                        Some("orcl".to_string()),
                                        None,
    );
    DbTypeWrapper::register_dbs(vec![db_config_oracle]).expect("Failed to register db");
}
