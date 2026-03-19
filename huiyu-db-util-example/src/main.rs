mod sqlite_test;
mod postgres_test;
mod mysql_test;
mod mappers;
mod entities;
mod datasource_test;
mod muti_test;
mod common;

use std::time;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    muti_test::test().await;
    sleep(time::Duration::from_millis(500)).await;
}


