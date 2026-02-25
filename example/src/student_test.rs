use r2d2_sqlite::SqliteConnectionManager;
use rustlog::{set_level, set_target, Level, Target};
use db_macros::Entity;
use db_mapper::base::config::DbConfig;
use db_mapper::base::db_type::DbType;
use db_mapper::pool::db_manager::DbManager;
use db_mapper::query::base_mapper::BaseMapper;

#[derive(Debug, Clone,Entity)]
#[table(name="t_student")]
pub struct StudentEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<i64>,
    #[field(column = "name")]
    pub name: Option<String>,
    #[field(column = "age")]
    pub age: Option<i64>,
}

pub struct StudentMapper;

impl BaseMapper<StudentEntity> for StudentMapper {}

#[tokio::test]
async fn test_student_mapper() {
    set_target(Target::Stderr);
    set_level(Level::Info);
    let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
    DbManager::register_db(&db_config, |db_config|{
        let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
        let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
        pool
    });
    let student_mapper = StudentMapper;
    let mut student_entity = StudentEntity {
        id: None,
        name: Some("张三".to_string()),
        age: Some(18),
    };
    let res = StudentMapper::insert(&mut student_entity).await;
    println!("insert res: {:?}", res.unwrap().unwrap());
}
