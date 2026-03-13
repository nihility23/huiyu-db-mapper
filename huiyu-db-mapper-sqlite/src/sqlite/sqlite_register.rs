use deadpool_sqlite::Config;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};

pub const SQLITE_DB_REGISTER: SqliteDbRegister = SqliteDbRegister;
pub struct SqliteDbRegister;

impl DbRegister for SqliteDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config,|config| {
            Config::new(config.database.clone().unwrap()).create_pool(deadpool_sqlite::Runtime::Tokio1).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))
        })?;
        Ok(())
    }

    fn check_config(&self,config: &DbConfig) -> Result<(), DatabaseError> {
        if config.database.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Database URL is missing".to_string()));
        }
        Ok(())
    }
}