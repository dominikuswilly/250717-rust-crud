mod controller;
mod service;
mod dto;
mod config;

use actix_web::{web, App, HttpServer};
use deadpool_postgres::Pool;
use crate::controller::{hello_controller, merk_controller};
use crate::config::db_config;
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); 
    info!("Starting server...");

    let pool: Pool = db_config::create_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(merk_controller::init_routes)
            .configure(hello_controller::init_routes)
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
