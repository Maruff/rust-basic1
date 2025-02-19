#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use my_user_api::{config, db, handlers}; // Import from your library
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file (if present)

    let config = config::load_config().expect("Failed to load configuration");
    let pool = db::create_pool(&config.database_url).expect("Failed to create pool");

    println!("Starting server at http://{}:{}", config.server_host, config.server_port);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .configure(handlers::user::configure) // Configure user routes
    })
    .bind((config.server_host, config.server_port))?
    .run()
    .await
}
