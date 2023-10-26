use crate::error::build_error;
use crate::lnd::LndClient;
use crate::users::load_users;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use lnurl::pay::LnURLPayInvoice;
use std::collections::HashMap;

pub async fn payment_request_response(Query(params): Query<HashMap<String, String>>) -> Response {
    let users = match load_users() {
        Ok(u) => u,
        Err(e) => {
            println!("Error: {}", e);
            return build_error("No file for users on server");
        }
    };

    let username = match params.get("username") {
        Some(name) => name,
        None => return build_error("Query in request must include username="),
    };

    let _amount = match params.get("amount") {
        Some(amt) => amt,
        None => return build_error("Query in request must include amount="),
    };

    let user = match users.get(username) {
        Some(u) => u,
        None => return build_error("No user on the server."),
    };

    let mut client = LndClient::new(
        user.address.clone(),
        user.cert.clone(),
        user.macaroon.clone(),
    )
    .await
    .unwrap();

    let payment_request = client.create_invoice(username.to_string()).await;

    let response = LnURLPayInvoice::new(payment_request);

    (StatusCode::OK, Json(response)).into_response()
}
