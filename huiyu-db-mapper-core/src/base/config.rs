use crate::base::db_type::DbType;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub db_type: DbType,
    pub name: String,
    pub host: Option<String>,
    pub port: Option<u32>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub schema: Option<String>,
    pub max_size: Option<u32>,
    pub min_size: Option<u32>,
    pub timeout: Option<u32>,

}

impl DbConfig {

    pub fn new(
        db_type: DbType,
        name: String,
        host: Option<String>,
        port: Option<u32>,
        username: Option<String>,
        password: Option<String>,
        database: Option<String>,
        schema: Option<String>,
    )-> Self {
        Self::new_with_all_opts(db_type, name, host, port, username, password, database, schema, None, None, None)
    }

    pub fn new_with_all_opts(
        db_type: DbType,
        name: String,
        host: Option<String>,
        port: Option<u32>,
        username: Option<String>,
        password: Option<String>,
        database: Option<String>,
        schema: Option<String>,
        max_size: Option<u32>,
        min_size: Option<u32>,
        timeout: Option<u32>,
    ) -> Self {
        Self {
            db_type,
            name,
            host,
            port,
            database,
            username,
            password,
            schema,
            max_size,
            min_size,
            timeout,
        }
    }
}
