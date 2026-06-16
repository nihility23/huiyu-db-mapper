use std::fmt::Display;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum DbType{
    Sqlite,
    Postgres,
    Mysql,
    Oracle,
    Dameng,
    Other,
}

impl Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DbType::Sqlite => write!(f, "sqlite"),
            DbType::Postgres => write!(f, "postgres"),
            DbType::Mysql => write!(f, "mysql"),
            DbType::Oracle => write!(f, "oracle"),
            DbType::Dameng => write!(f, "dameng"), 
            // DbType::SqlServer => write!(f, "sqlserver"),
            DbType::Other => write!(f, "other"),
        }
    }
}