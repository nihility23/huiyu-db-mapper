use mysql_async::Pool;
use tracing::info;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};

pub const MYSQL_DB_REGISTER: MysqlDbRegister = MysqlDbRegister;
pub struct MysqlDbRegister;
impl DbRegister for MysqlDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config, |config| {
            // 1. 创建数据库连接池
            // mysql://root:password@localhost:3307/db_name
            // let url = format!("mysql://{}:{}@{}:{}/{}", config.username.clone().unwrap(), config.password.clone().unwrap(), config.host.clone().unwrap(), config.port.clone().unwrap(), config.database.clone().unwrap());
            info!("mysql url: {}", config.url.as_ref().unwrap().as_str());
            Ok(Pool::new(config.url.as_ref().unwrap().as_str()))
        })?;
        Ok(())
    }

}