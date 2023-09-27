mod error;
mod lnd;
mod pay;
mod db;
mod users;

use axum::{routing::get, Router, Extension};
use pay::{payment_request_callback, payment_request_response};
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() {

    let database = match SqlitePoolOptions::new().connect("benlnurl.db").await {
        Ok(db) => db,
        Err(e) => {
            println!("Could not connect to database: {}", e);
            panic!();
        }
    };

    println!("Starting benlnurl server");

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    let router = Router::new()
        .route("/", get(payment_request_callback))
        .route("/pay", get(payment_request_response))
        .layer(Extension(database));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}


