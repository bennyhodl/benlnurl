mod error;
mod lnd;
mod pay;
mod db;

use axum::{routing::get, Router};
use pay::payment_request_callback;
use db::connect_to_database;

#[tokio::main]
async fn main() {

    let db = connect_to_database();

    println!("Starting benlnurl server");

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    let router = Router::new()
        .route("/", get(payment_request_callback));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}


