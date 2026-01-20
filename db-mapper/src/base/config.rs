use crate::base::db_type::DbType;

#[derive(Debug,Clone)]
pub struct DbConfig{
    pub host:Option<String>,
    pub port:Option<u32>,
    pub database:Option<String>,
    pub username:Option<String>,
    pub password:Option<String>,
    pub schema:Option<String>,
    pub name:String,
    pub db_type: DbType,
}

impl DbConfig{
    pub fn new(db_type: DbType, host:Option<String>,port:Option<u32>,
                  database:Option<String>,username:Option<String>,password:Option<String>,
                  schema: Option<String>, name:String)->Self{
        Self{
            host,
            port,
            database,
            username,
            password,
            schema,
            name,
            db_type
        }
    }
}