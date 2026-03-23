use tracing::info;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};
use oracle_rs::Config;
use deadpool_oracle::{Pool, PoolBuilder};

pub const ORACLE_DB_REGISTER: OracleDbRegister = OracleDbRegister;
pub struct OracleDbRegister;
impl DbRegister for OracleDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config, |config| {
            // Create connection config
            let config = Config::new(config.host.unwrap_or("localhost".to_string()),
                                     config.port.unwrap_or(1521),
                                     config.database.unwrap_or("orcl".to_string()),
                                     config.username.unwrap(),
                                     config.password.unwrap(),);

            // Create pool
            let pool = PoolBuilder::new(config)
                .max_size(10)
                .build()?;
            pool
        })?;
        Ok(())
    }

}