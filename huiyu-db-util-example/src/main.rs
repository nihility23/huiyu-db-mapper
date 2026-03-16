mod sqlite_test;
mod postgres_test;
mod mysql_test;
mod mappers;
mod entities;

use std::time;
use tokio::time::sleep;

#[tokio::main]
async fn main() {

    sleep(time::Duration::from_millis(5)).await;
}


