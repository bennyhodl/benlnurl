mod error;
mod lnd;
mod pay;
mod users;
mod util;

use axum::{routing::get, Router};
use pay::payment_request_response;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    let router = Router::new().route("/pay", get(payment_request_response));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}
