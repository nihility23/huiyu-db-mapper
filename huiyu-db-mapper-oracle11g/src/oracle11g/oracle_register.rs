use std::str::FromStr;
use r2d2_oracle::OracleConnectionManager;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};

pub const ORACLE11G_DB_REGISTER: Oracle11gDbRegister = Oracle11gDbRegister;
pub struct Oracle11gDbRegister;
impl DbRegister for Oracle11gDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config, |config| {
            // Formats supported:
            // - `host:port/service_name`
            // - `host/service_name`
            // - `host:port:sid`
            // - `//host:port/service_name` (with optional leading slashes)
            let manager = OracleConnectionManager::new(config.username.clone().unwrap().as_str(), config.password.clone().unwrap().as_str(), config.url.clone().unwrap().as_str());

            // Create pool
            r2d2::Pool::builder()
                .max_size(config.max_size.unwrap_or(10) as u32)
                .idle_timeout(Some(std::time::Duration::from_secs(config.timeout.unwrap_or(15).into())))
                .build(manager).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))
        })?;
        Ok(())
    }

}