use db_macros::Entity;
use db_mapper::base::config::DbConfig;
use db_mapper::base::db_type::DbType;
use db_mapper::pool::db_manager::DbManager;
use db_mapper::query::base_mapper::BaseMapper;
use db_mapper::query::query_wrapper::QueryWrapper;
use r2d2_sqlite::SqliteConnectionManager;
use rustlog::error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug,Entity,Serialize,Deserialize)]
#[table(name = "t_app")]
pub struct AppEntity{
    #[id(column = "id", auto_increment = true)]
    pub id: Option<String>,
    #[field(column = "app_name")]
    pub app_name: Option<String>,
    #[field(column = "app_key")]
    pub app_key: Option<String>,
    #[field(column = "app_secret")]
    pub app_secret: Option<String>,
    #[field(column = "create_time")]
    pub create_time: Option<i64>,
}

// impl Entity for AppEntity {
//     type K = String;
//
//     fn key(&self) -> Self::K {
//         self.id.clone().unwrap_or_default()
//     }
//
//     fn key_name() -> &'static str {
//         "id"
//     }
//
//     fn column_names() -> Vec<&'static str> {
//         vec!["id", "app_name", "app_key", "app_secret", "create_time"]
//     }
//
//     fn field_names() -> Vec<&'static str> {
//         vec!["id", "app_name", "app_key", "app_secret", "create_time"]
//     }
//
//     fn table_name() -> &'static str {
//         "t_app"
//     }
//
//     fn new() -> Self {
//         AppEntity {
//             id: None,
//             app_name: None,
//             app_key: None,
//             app_secret: None,
//             create_time: None,
//         }
//     }
//
//     fn get_value_by_field_name(&self, field_name: &str) -> ParamValue {
//         match field_name {
//             "id" => if self.id.is_none() { ParamValue::Null } else { self.id.clone().unwrap().into() },
//             "app_name" => if self.app_name.is_none() { ParamValue::Null } else { self.app_name.clone().unwrap().into() },
//             "app_key" => if self.app_key.is_none() { ParamValue::Null } else { self.app_key.clone().unwrap().into() },
//             "app_secret" => if self.app_secret.is_none() { ParamValue::Null } else { self.app_secret.clone().unwrap().into() },
//             "create_time" => if self.create_time.is_none() { ParamValue::Null } else { self.create_time.clone().unwrap().into() },
//             _ => ParamValue::Null,
//         }
//     }
//
//     fn get_value_by_column_name(&self, column_name: &str) -> ParamValue {
//         match column_name {
//             "id" => if self.id.is_none() { ParamValue::Null } else { self.id.clone().unwrap().into() },
//             "app_name" => if self.app_name.is_none() { ParamValue::Null } else { self.app_name.clone().unwrap().into() },
//             "app_key" => if self.app_key.is_none() { ParamValue::Null } else { self.app_key.clone().unwrap().into() },
//             "app_secret" => if self.app_secret.is_none() { ParamValue::Null } else { self.app_secret.clone().unwrap().into() },
//             "create_time" => if self.create_time.is_none() { ParamValue::Null } else { self.create_time.clone().unwrap().into() },
//             _ => ParamValue::Null,
//         }
//     }
//
//     fn set_value_by_field_name(&mut self, field_name: &str, value: ParamValue) {
//         match field_name {
//             "id" => self.id = Some(value.to_string()),
//             "app_name" => self.app_name = Some(value.to_string()),
//             "app_key" => self.app_key = Some(value.to_string()),
//             "app_secret" => self.app_secret = Some(value.to_string()),
//             "create_time" => self.create_time = value.into(),
//             _ => error!("Field name not found: {}", field_name),
//         }
//     }
//
//     fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue) {
//         self.set_value_by_field_name(column_name, value)
//     }
//
//     fn get_column_infos() -> Vec<ColumnInfo> {
//         vec![
//             ColumnInfo::new("id", "id", ColumnType::String, false, false, true),
//             ColumnInfo::new("app_name", "app_name", ColumnType::String, false, false, false),
//             ColumnInfo::new("app_key", "app_key", ColumnType::String, false, false, false),
//             ColumnInfo::new("app_secret", "app_secret", ColumnType::String, false, false, false),
//             ColumnInfo::new("create_time", "create_time", ColumnType::I64, false, false, false),
//         ]
//     }
// }

pub struct AppMapper;

impl BaseMapper<AppEntity> for AppMapper {}

#[cfg(test)]
mod tests{
    use super::*;
    use db_mapper::base::config::DbConfig;
    use db_mapper::base::db_type::DbType;
    use db_mapper::pool::db_manager::DbManager;
    use db_mapper::query::query_wrapper::QueryWrapper;
    use r2d2_sqlite::SqliteConnectionManager;
    use db_mapper::base::param::ParamValue;

    #[test]
    fn test(){
        let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
        DbManager::register_db(&db_config, |db_config|{
            let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
            let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
            pool
        });

        let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
        let res = AppMapper::select_one(&query_wrapper);
    }
}

pub async fn test(){
    let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
    DbManager::register_db(&db_config, |db_config|{
    let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
    pool
    });

    // query one
    let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("3".to_string()));
    let res = AppMapper::select_one(&query_wrapper).await;
    if res.is_err(){
        error!("Error: {}", res.err().unwrap());
    }else {

        let value = res.unwrap();
        println!("select one {}", serde_json::to_string_pretty(&value).unwrap());
    }

    // query list
    let query_wrapper = QueryWrapper::new().like("app_name", ParamValue::String("f".to_string()));
    let res = AppMapper::select(&query_wrapper).await;
    if res.is_err(){
        error!("Error: {}", res.err().unwrap());
    }else {
        let value = res.unwrap();
        println!("query list {}", serde_json::to_string_pretty(&value).unwrap());
    }

    // select_by_key
    let res = AppMapper::select_by_key(&"2".to_string()).await;
    let value = res.unwrap();
    println!("select_by_key {}", serde_json::to_string_pretty(&value).unwrap());

    // update by key
    let mut entity = AppEntity::new();
    entity.app_secret = Some(uuid::Uuid::new_v4().to_string().replace("-", ""));
    entity.id = Some("2".to_string());
    let res = AppMapper::update_by_key(&entity).await;
    println!("update by key {:?}", json!(res.unwrap()));

    // delete by key
    let res = AppMapper::delete_by_key(&"2".to_string()).await;
    println!("delete by key {:?}", json!(res.unwrap()));

    // delete by wrapper
    let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
    let res = AppMapper::delete(&query_wrapper).await;
    println!("delete by wrapper {:?}", json!(res.unwrap()));

    // insert
    let mut entity = AppEntity::new();
    entity.id = Some("13".to_string());
    entity.app_name = Some("test".to_string());
    entity.app_key = Some("test".to_string());
    entity.app_secret = Some("test".to_string());
    entity.create_time = Some(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64);
    let res = AppMapper::insert(&mut entity).await;
    println!("insert {:?}", json!(res.unwrap()));

}