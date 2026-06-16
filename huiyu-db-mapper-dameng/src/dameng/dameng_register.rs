use dameng_rust_sdk::{ConnectionOptions, DamengClient};
use tracing::info;
use huiyu_db_mapper_core::base::config::DbConfig;
use huiyu_db_mapper_core::base::error::DatabaseError;
use huiyu_db_mapper_core::pool::db_manager::{DbManager, DbRegister};
use url::Url;

pub const DAMENG_DB_REGISTER: DamengDbRegister = DamengDbRegister;
pub struct DamengDbRegister;
impl DbRegister for DamengDbRegister{
    fn register_db(&self, config: &DbConfig) -> Result<(), DatabaseError> {
        Self::check_config(self, config)?;
        DbManager::register(config, |config| {
            // dm://10.150.1.88:5234?schema=MEDILINK&useUnicode=true&characterEncoding=utf-8&useSSL=false&serverTimezone=Asia/Shanghai
            info!("dameng url: {}", config.url.as_ref().unwrap().as_str());
            // Create connection options
            let mut options = DamengDbRegister::parse_from_url(config.url.as_ref().unwrap().as_str())?;
            options.username = config.username.clone().unwrap_or_default();
            options.password = config.password.clone().unwrap_or_default();
            // 2. 从基础配置创建Builder，再单独设置用户名和密码
            // Create client
            let client = DamengClient::new(options).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))?;
            Ok(client)
        })?;
        Ok(())
    }

}

impl DamengDbRegister {
    fn parse_from_url(url: &str) -> Result<ConnectionOptions, DatabaseError> {
        // dm://10.150.1.88:5234?schema=MEDILINK&useUnicode=true&characterEncoding=utf-8&useSSL=false&serverTimezone=Asia/Shanghai
        // 清洗字符串：去掉可能的 "// " 或 "//" 前缀
        let cleaned = url.trim();
        let cleaned = if cleaned.starts_with("// dm://") {
            cleaned.replacen("// dm://", "dm://", 1)
        } else if cleaned.starts_with("//") && !cleaned.starts_with("//dm://") {
            cleaned.replacen("//", "dm://", 1)
        } else {
            cleaned.to_string()
        };

        let url = url::Url::parse(&cleaned).map_err(|e| DatabaseError::PoolCreateError(e.to_string()))?;
        if url.scheme() != "dm" {
            return Err(DatabaseError::PoolCreateError(format!("不支持的协议: {}, 期望 'dm'", url.scheme())));
        }

        // 提取 server 和 port
        let server = url.host_str().ok_or(DatabaseError::PoolCreateError("缺少 host".to_string()))?.to_string();
        let port = url.port().ok_or(DatabaseError::PoolCreateError("缺少端口".to_string()))?;

        // 提取用户名/密码（如果 URL 中包含 user:pass@）
        let (username, password) = match url.username() {
            "" => (String::new(), String::new()),
            user => {
                let pwd = url.password().unwrap_or("");
                (user.to_string(), pwd.to_string())
            }
        };

        // 解析查询参数
        let mut schema = String::new();
        let mut timeout = 30; // 默认超时 30 秒
        let mut use_tls = false;
        let mut additional = Vec::new();
        let mut driver = "DM".to_string(); // 默认驱动

        for (key, value) in url.query_pairs() {
            match key.as_ref() {
                "schema" => schema = value.to_string(),
                "timeout" => {
                    timeout = value.parse::<u32>().unwrap_or(30);
                }
                "useSSL" => {
                    // 将 useSSL 映射到 use_tls
                    use_tls = value.parse::<bool>().unwrap_or(false);
                }
                "driver" => driver = value.to_string(),
                // 其他所有参数都放入 additional_params
                _ => {
                    additional.push((key.to_string(), value.to_string()));
                }
            }
        }

        // 如果 schema 依然为空，可以尝试从 URL path 获取（例如 dm://host:port/schema）
        if schema.is_empty() && url.path().len() > 1 {
            schema = url.path()[1..].to_string();
        }

        Ok(ConnectionOptions {
            server,
            port,
            username,
            password,
            schema,
            timeout,
            use_tls,
            additional_params: additional,
            driver,
        })
    }
}