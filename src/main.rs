// External
#[macro_use]
extern crate diesel;

// Dependencies

use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

// Modules

mod errors;
mod handlers;
mod models;
mod schema;

// Helper functions

// Main function

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create db connection pool
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    let pool = web::Data::new(pool);

    // Start http server
    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .route("/users", web::get().to(handlers::get_users))
            .route("/users", web::post().to(handlers::add_user))
            .route("/users/{id}", web::get().to(handlers::get_user_by_id))
            .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
