
mod common;
mod controller;
mod param;
mod mapper;
mod test;
mod entity;

use actix_web::{web, App, HttpServer};
use crate::common::db::init_dbs;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    // 初始化 tracing
    tracing_subscriber::fmt::init();
    init_dbs();
    HttpServer::new(move || {
        App::new()
            .route("/user/delete_user", web::post().to(controller::user_controller::delete_user))
            .route("/user/save_user", web::post().to(controller::user_controller::save_user))
            .route("/user/query_user_page", web::post().to(controller::user_controller::query_user_page))
            .route("/user/query_user_by_id/{id}", web::post().to(controller::user_controller::query_user_by_id))
            .route("/user/query_user_name_by_id/{id}", web::get().to(controller::user_controller::query_user_name_by_id))
            .route("/user/query_user_name_by_page", web::get().to(controller::user_controller::query_user_name_by_page))
    })
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}


