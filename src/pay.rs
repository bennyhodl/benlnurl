use crate::db::BenlnurlDatabase;
use crate::error::build_error;
use crate::users::load_users;
use crate::{error::BenlnurlError, lnd::LndClient};
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashMap;
use std::{fs, sync::Arc};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    paymentRequest: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct BenlnurlPayCallback {
    callback: String, // The URL from LN SERVICE which will accept the pay request parameters
    maxSendable: u64, // Max millisatoshi amount LN SERVICE is willing to receive
    minSendable: u64, // Min millisatoshi amount LN SERVICE is willing to receive, can not be less than 1 or more than `maxSendable`
    metadata: String, // Metadata json which must be presented as raw string here, this is required to pass signature verification at a later step
    tag: String,      // Type of LNURL
}

#[derive(Serialize, Deserialize)]
pub struct BenlnurlPayResponse {
    pr: String,
    routes: Vec<String>,
}

pub async fn payment_request_callback(Query(params): Query<HashMap<String, String>>) -> Response {
    // get connection info from request
    let username = match params.get("username") {
        Some(username) => username,
        None => return build_error("Query param needs to be username="),
    };

    // look for user in db.

    (
        StatusCode::OK,
        Json(BenlnurlPayCallback {
            callback: format!("http://localhost:3000/payRequest?username={}", username),
            maxSendable: 50000,
            minSendable: 10,
            metadata: "[[\"text/plain\", \"Pay Ben!\"]]".to_string(),
            tag: "paymentRequest".to_string(),
        }),
    )
        .into_response()
}

pub async fn payment_request_response(Query(params): Query<HashMap<String, String>>) -> Response {
    let users = match load_users() {
        Ok(u) => u,
        Err(e) => {
            println!("Error: {}", e);
            return build_error("No file for users on server");
        }
    };

    info!("Loaded users: {:?}", users);

    let username = match params.get("username") {
        Some(name) => name,
        None => return build_error("Query in request must include username="),
    };

    let amount = match params.get("amount") {
        Some(amt) => amt,
        None => return build_error("Query in request must include amount="),
    };

    info!("Using username: {} for amount {}", username, amount);

    let user = match users.get(username) {
        Some(u) => u,
        None => return build_error("No user on the server."),
    };

    println!("Got user: {:?}", user);

    let mut client = LndClient::new(
        user.address.clone(),
        user.cert.clone(),
        user.macaroon.clone(),
    )
    .await
    .unwrap();

    let payment_request = client.create_invoice().await;

    (
        StatusCode::OK,
        Json(BenlnurlPayResponse {
            pr: payment_request,
            routes: Vec::new(),
        }),
    )
        .into_response()
}
