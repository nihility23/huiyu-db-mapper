mod sqlite_test;
mod postgres_test;
mod mysql_test;
mod mappers;
mod entities;

use std::time;
use rustlog::{set_level, set_target, Level, Target};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    set_target(Target::Stderr);
    set_level(Level::Info);
    
    sleep(time::Duration::from_millis(5)).await;
}


