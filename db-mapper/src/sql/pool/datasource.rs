use std::cell::RefCell;
use std::collections::HashMap;
use r2d2::{ManageConnection, Pool};
use r2d2_mysql::MySqlConnectionManager;

pub(crate) struct SqlExecutor<T: r2d2::ManageConnection> {
    pub(crate) pool: Pool<T>
}
