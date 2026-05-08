use mysql_async::{Opts, OptsBuilder, Pool};
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
            // mysql://localhost:3307/db_name
            info!("mysql url: {}", config.url.as_ref().unwrap().as_str());
            let base_opts = Opts::from_url(config.url.as_ref().unwrap().as_str()).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))?;

            // 2. 从基础配置创建Builder，再单独设置用户名和密码
            let opts = OptsBuilder::from_opts(base_opts)
                .user(Some(config.username.clone().unwrap()))
                .pass(Some(config.password.clone().unwrap()));
            let pool = Pool::new(opts);
            Ok(pool)
        })?;
        Ok(())
    }

}