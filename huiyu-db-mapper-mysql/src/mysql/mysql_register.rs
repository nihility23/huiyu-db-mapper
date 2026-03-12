use mysql::Pool;
use rustlog::info;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};

pub struct MysqlDbRegister;
impl DbRegister for MysqlDbRegister{
    fn register_db(config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(config)?;
        DbManager::register(config, |config| {
            let url = format!("mysql://{}:{}@{}:{}/{}", config.username.clone().unwrap(), config.password.clone().unwrap(), config.host.clone().unwrap(), config.port.clone().unwrap(), config.database.clone().unwrap());
            info!("mysql url: {}", url);
            Pool::new(url.as_str()).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))
        })?;
        Ok(())
    }

}