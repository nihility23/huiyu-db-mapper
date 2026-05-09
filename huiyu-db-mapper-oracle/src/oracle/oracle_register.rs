use std::str::FromStr;
use deadpool_oracle::PoolBuilder;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};
use oracle_rs::Config;

pub const ORACLE_DB_REGISTER: OracleDbRegister = OracleDbRegister;
pub struct OracleDbRegister;
impl DbRegister for OracleDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config, |config| {
            // Formats supported:
            // - `host:port/service_name`
            // - `host/service_name`
            // - `host:port:sid`
            // - `//host:port/service_name` (with optional leading slashes)
            // Create connection config
            let mut inner_config = Config::from_str(config.clone().url.unwrap().as_str()).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))?;
            inner_config.set_username(config.username.clone().unwrap());
            inner_config.set_password(config.password.clone().unwrap());
            

            // Create pool

            PoolBuilder::new(inner_config)
                .max_size(config.max_size.unwrap_or(10) as usize)
                .create_timeout(Some(std::time::Duration::from_secs(config.timeout.unwrap_or(1).into())))
                .build().map_err(|e| DatabaseError::PoolCreateError(e.to_string()))
        })?;
        Ok(())
    }

}