mod db;
mod error;
mod lnd;
mod pay;
mod users;

use axum::{routing::get, Router};
use pay::{payment_request_callback, payment_request_response};

#[tokio::main]
async fn main() {
    println!("Starting benlnurl server");

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    let router = Router::new()
        .route("/", get(payment_request_callback))
        .route("/pay", get(payment_request_response));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
