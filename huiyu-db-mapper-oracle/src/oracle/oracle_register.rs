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
            // Create connection config
            let mut inner_config = Config::from_str(config.clone().url.unwrap().as_str()).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))?;
            inner_config.set_username(config.username.clone().unwrap());
            inner_config.set_password(config.password.clone().unwrap());
            

            // Create pool

            PoolBuilder::new(inner_config)
                .max_size(10)
                .build().map_err(|e| DatabaseError::CommonError(e.to_string()))
        })?;
        Ok(())
    }

}