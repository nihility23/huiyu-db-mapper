use crate::base::config::DbConfig;
use crate::base::error::DatabaseError;
use crate::pool::db_manager::{DbManager, DbRegister};
use deadpool_sqlite::Config;

pub const SQLITE_REGISTER: SqliteDbRegister = SqliteDbRegister;
pub struct SqliteDbRegister;

impl DbRegister for SqliteDbRegister{
    fn register_db(config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(config)?;
        DbManager::register(config,|config| {
            Config::new(config.database.clone().unwrap()).create_pool(deadpool_sqlite::Runtime::Tokio1).expect("Failed to create pool")
        })?;
        Ok(())
    }

    fn check_config(config: &DbConfig) -> Result<(), DatabaseError> {
        if config.database.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Database URL is missing".to_string()));
        }
        Ok(())
    }
}