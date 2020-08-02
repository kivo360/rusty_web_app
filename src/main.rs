#[macro_use]
extern crate diesel;
extern crate lazy_static;

pub mod config;
pub mod handles;
pub mod models;
pub mod schema;
pub mod util;

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::io;

// Creating a connection Pool type for easier references
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!(
        "Starting new server at http://{}:{}/",
        config.server.host, config.server.port
    );
    let bind_address = format!("{}:{}", config.server.host, config.server.port);
    // TODO: Replace the DATABASE_URL
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(crate::handles::status))
            .route("/register", web::post().to(crate::handles::register_user))
            .route("/login", web::get().to(crate::handles::login_user))
            .route("/logout", web::post().to(crate::handles::logout_user))
            // Gets the latest user
            .route("/latest", web::get().to(crate::handles::delete_user))
            // Updates generic information for the user.
            // Most commonly used to update the favorite streamer.
            // Can update other things like keys too.
            .route("/update", web::get().to(crate::handles::delete_user))
    })
    .bind(bind_address)?
    .run()
    .await
}
