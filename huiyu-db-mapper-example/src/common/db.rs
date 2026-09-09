use huiyu_db_mapper::huiyu_db_mapper_impl::query::db_type_wrapper::DbTypeWrapper;
use huiyu_db_mapper::huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper::huiyu_db_mapper_core::base::db_type::DbType;

pub fn init_dbs(){
    // init_postgres();
    // init_mysql();
    // init_sqlite();
    // init_oracle();
    init_oracle11g();
    // init_dameng();
}

fn init_postgres(){
    println!("init postgres");
//     postgresql://user:pass@localhost:5432/mydb?sslmode=require
    let db_config_postgres = DbConfig::new(DbType::Postgres,
                                           "postgres".to_string(),
                                           Some("postgresql://user:pass@localhost:5432/dbname".to_string()),
                                           None,
                                           None,
    );
    DbTypeWrapper::register_dbs(vec![db_config_postgres]).expect("Failed to register db");
}
fn init_mysql(){
    println!("init mysql");
    // "mysql://root:password@localhost:3306/mydb"
    let db_config_mysql = DbConfig::new(DbType::Mysql,
                                        "mysql".to_string(),
                                        Some("mysql://root:123456!@localhost:3306/dbname".to_string()),
                                        None,None
    );
    DbTypeWrapper::register_dbs(vec![db_config_mysql]).expect("Failed to register db");
}
fn init_sqlite(){
    println!("init sqlite");
    let db_config_sqlite = DbConfig::new(
        DbType::Sqlite,
        "sqlite".to_string(),
        Some("E:\\test\\dbname.db".to_string()),
        None,
        None
    );
    DbTypeWrapper::register_dbs(vec![db_config_sqlite]).expect("Failed to register db");
}

fn init_oracle(){
    println!("init oracle");
    let db_config_oracle = DbConfig::new(DbType::Oracle,
                                        "oracle".to_string(),
                                        Some("localhost:1521/orcl".to_string()),
                                        Some("user".to_string()),Some("password".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_oracle]).expect("Failed to register db");
}

fn init_oracle11g(){
    println!("init oracle11g");
    let db_config_oracle11g = DbConfig::new(DbType::Oracle11g,
                                            "oracle11g".to_string(),
                                            Some("localhost:1521/orcl".to_string()),
                                            Some("user".to_string()),Some("password".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_oracle11g]).expect("Failed to register db");
}

fn init_dameng(){
    println!("init dameng");
    let db_config_dameng = DbConfig::new(DbType::Dameng,
                                        "dameng".to_string(),
                                        Some("dm://localhost:5234?schema=MEDILINK&useUnicode=true&characterEncoding=utf-8&useSSL=false&serverTimezone=Asia/Shanghai".to_string()),
                                        Some("user".to_string()),Some("password".to_string()),
    );
    DbTypeWrapper::register_dbs(vec![db_config_dameng]).expect("Failed to register db");
}