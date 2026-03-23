use std::fmt::Display;

#[derive(Debug,Clone,Copy)]
pub enum DbType{
    Sqlite,
    Postgres,
    Mysql,
    Oracle,

    Other,
}

impl Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            DbType::Sqlite => write!(f, "sqlite"),
            DbType::Postgres => write!(f, "postgres"),
            DbType::Mysql => write!(f, "mysql"),
            DbType::Oracle => write!(f, "oracle"),
            // DbType::SqlServer => write!(f, "sqlserver"),
            DbType::Other => write!(f, "other"),
        }
    }
}