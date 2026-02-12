use r2d2_sqlite::SqliteConnectionManager;
use db_mapper::base::config::DbConfig;
use db_mapper::base::db_type::DbType;
use db_mapper::base::entity::{ColumnInfo, ColumnType, Entity};
use db_mapper::base::param::ParamValue;
use db_mapper::pool::db_manager::DbManager;
use db_mapper::query::base_mapper::BaseMapper;
use db_mapper::query::query_wrapper::QueryWrapper;

#[derive(Clone, Debug)]
pub struct AppEntity{
    pub id: Option<String>,
    pub app_name: Option<String>,
    pub app_key: Option<String>,
    pub app_secret: Option<String>,
    pub create_time: Option<i64>,
}


impl Entity for AppEntity {
    type K = String;

    fn key(&self) -> Self::K {
        self.id.clone().unwrap_or_default()
    }

    fn key_name() -> &'static str {
        "id"
    }

    fn column_names() -> Vec<&'static str> {
        vec!["id", "app_name", "app_key", "app_secret", "create_time"]
    }

    fn field_names() -> Vec<&'static str> {
        vec!["id", "app_name", "app_key", "app_secret", "create_time"]
    }

    fn table_name() -> &'static str {
        "t_app"
    }

    fn new() -> Self {
        AppEntity {
            id: None,
            app_name: None,
            app_key: None,
            app_secret: None,
            create_time: None,
        }
    }

    fn get_value_by_field_name(&self, field_name: &str) -> ParamValue {
        match field_name {
            "id" => ParamValue::String(self.id.clone().unwrap_or_default()),
            "app_name" => ParamValue::String(self.app_name.clone().unwrap_or_default()),
            "app_key" => ParamValue::String(self.app_key.clone().unwrap_or_default()),
            "app_secret" => ParamValue::String(self.app_secret.clone().unwrap_or_default()      ),
            "create_time" => ParamValue::I64(self.create_time.unwrap_or_default()),
            _ => panic!("Field name not found: {}", field_name),
        }
    }

    fn get_value_by_column_name(&self, column_name: &str) -> ParamValue {
        match column_name {
            "id" => if self.id.is_none() { ParamValue::Null } else { ParamValue::String(self.id.clone().unwrap_or_default()) },
            "app_name" => if self.app_name.is_none() { ParamValue::Null } else { ParamValue::String(self.app_name.clone().unwrap_or_default()) },
            "app_key" => if self.app_key.is_none() { ParamValue::Null } else { ParamValue::String(self.app_key.clone().unwrap_or_default()) },
            "app_secret" => if self.app_secret.is_none() { ParamValue::Null } else { ParamValue::String(self.app_secret.clone().unwrap_or_default()) },
            "create_time" => if self.create_time.is_none() { ParamValue::Null } else { ParamValue::I64(self.create_time.unwrap_or_default()) },
            _ => panic!("Column name not found: {}", column_name),
        }
    }

    fn set_value_by_field_name(&mut self, field_name: &str, value: ParamValue) {
        match field_name {
            "id" => self.id = Some(value.to_string()),
            "app_name" => self.app_name = Some(value.to_string()),
            "app_key" => self.app_key = Some(value.to_string()),
            "app_secret" => self.app_secret = Some(value.to_string()),
            "create_time" => self.create_time = value.into(),
            _ => panic!("Field name not found: {}", field_name),
        }
    }

    fn set_value_by_column_name(&mut self, column_name: &str, value: ParamValue) {
        self.set_value_by_field_name(column_name, value)
    }

    fn get_column_infos() -> Vec<ColumnInfo> {
        vec![
            ColumnInfo::new("id", "id", ColumnType::String, false, false, true),
            ColumnInfo::new("app_name", "app_name", ColumnType::String, false, false, false),
            ColumnInfo::new("app_key", "app_key", ColumnType::String, false, false, false),
            ColumnInfo::new("app_secret", "app_secret", ColumnType::String, false, false, false),
            ColumnInfo::new("create_time", "create_time", ColumnType::I64, false, false, false),
        ]
    }
}

pub struct AppMapper;

impl BaseMapper<AppEntity> for AppMapper {}

#[cfg(test)]
mod tests{
    use r2d2_sqlite::SqliteConnectionManager;
    use db_mapper::base::config::DbConfig;
    use db_mapper::base::db_type::DbType;
    use db_mapper::pool::db_manager::DbManager;
    use db_mapper::query::query_wrapper::QueryWrapper;
    use super::*;

    #[test]
    fn test(){
        let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
        DbManager::register_db(&db_config, |db_config|{
            let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
            let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
            pool
        });

        let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
        let res = AppMapper{}.select_one(&query_wrapper);
    }
}

pub async fn test(){
    let db_config = DbConfig::new(DbType::Sqlite, None, None, Some("E:\\test\\tiny-file-manager\\db\\tiny-file-manager.db".to_string()), None, None, None, "default".to_string());
    DbManager::register_db(&db_config, |db_config|{
    let manager = SqliteConnectionManager::file(db_config.database.clone().unwrap().to_string());
    let pool = r2d2::Pool::new(manager).expect("Failed to create pool");
    pool
    });

    let app_mapper = AppMapper{};
    // query one
    let query_wrapper = QueryWrapper::new().eq("id", ParamValue::String("1".to_string()));
    let res = app_mapper.select_one(&query_wrapper).await;
    println!("select one {:?}", res.unwrap());

    // query list
    let query_wrapper = QueryWrapper::new().like("app_name", ParamValue::String("f".to_string()));
    let res = app_mapper.select(&query_wrapper).await;
    println!("select list {:?}", res.unwrap());

    // select_by_key
    let res = app_mapper.select_by_key(&"2".to_string()).await;
    println!("select by key {:?}", res.unwrap());

    // update by key
    let mut entity = AppEntity::new();
    entity.app_secret = Some(uuid::Uuid::new_v4().to_string().replace("-", ""));
    entity.id = Some("2".to_string());
    let res = app_mapper.update_by_key(&entity).await;
    println!("update by key {:?}", res.unwrap());
}