use crate::base::db_type::DbType;

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub db_type: DbType,
    pub name: String,
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub max_size: Option<u16>,
    pub min_size: Option<u16>,
    pub timeout: Option<u16>,

}

impl DbConfig {

    pub fn new(
        db_type: DbType,
        name: String,
        url: Option<String>,
        username: Option<String>,
        password: Option<String>,
    )-> Self {
        Self::new_with_all_opts(db_type, name, url, username, password, None, None, None)
    }

    pub fn new_with_all_opts(
        db_type: DbType,
        name: String,
        url: Option<String>,
        username: Option<String>,
        password: Option<String>,
        max_size: Option<u16>,
        min_size: Option<u16>,
        timeout: Option<u16>,
    ) -> Self {
        Self {
            db_type,
            name,
            url,
            username,
            password,
            max_size,
            min_size,
            timeout,
        }
    }
}
