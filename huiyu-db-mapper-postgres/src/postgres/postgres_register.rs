use deadpool_postgres::{ManagerConfig, RecyclingMethod, Runtime};
use tokio_postgres::NoTls;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};

pub struct PostgresDbRegister;

impl DbRegister for PostgresDbRegister{
    fn register_db(config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(config)?;
        DbManager::register(config, |config| {
            let mut cfg = deadpool_postgres::Config::new();
            cfg.dbname = Some(config.database.clone().expect("Database name is missing").to_string());
            cfg.manager = Some(ManagerConfig {
                recycling_method: RecyclingMethod::Fast,
            });
            cfg.user = Some(config.username.clone().expect("Username is missing").to_string());
            cfg.password = Some(config.password.clone().expect("Password is missing").to_string());
            cfg.host = Some(config.host.clone().expect("Host is missing").to_string());
            cfg.port = Some(config.port.expect("Port is missing") as u16);
            if config.schema.is_some() {
                cfg.options = Some(format!("--search_path={}",config.schema.clone().unwrap()));
            }
            cfg.create_pool(Some(Runtime::Tokio1), NoTls ).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))
        })?;
        Ok(())
    }

    fn check_config(config: &DbConfig) -> Result<(), DatabaseError> {
        if config.database.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Database is missing".to_string()));
        }
        if config.username.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Username is missing".to_string()));
        }
        if config.password.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Password is missing".to_string()));
        }
        if config.host.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Host is missing".to_string()));
        }
        if config.port.is_none() {
            return Err(DatabaseError::ConfigNotFoundError("Port is missing".to_string()));
        }
        Ok(())
    }
}