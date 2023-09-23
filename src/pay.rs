use crate::error::BenlnurlError;
use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct PaymentRequest {
    paymentRequest: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct BenlnurlPayCallback {
    callback: String, // The URL from LN SERVICE which will accept the pay request parameters
    maxSendable: u64, // Max millisatoshi amount LN SERVICE is willing to receive
    minSendable: u64, // Min millisatoshi amount LN SERVICE is willing to receive, can not be less than 1 or more than `maxSendable`
    metadata: String, // Metadata json which must be presented as raw string here, this is required to pass signature verification at a later step
    tag: String,      // Type of LNURL
}

pub async fn payment_request_callback(Query(params): Query<HashMap<String, String>>) -> Response {
    // get connection info from request
    let username = match params.get("username") {
        Some(username) => username,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(BenlnurlError {
                    status: "ERROR".to_string(),
                    reason: "Query in request must be <url>?username=".to_string(),
                }),
            )
                .into_response()
        }
    };

    // look for user in db.
    // if not found 404: No user

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
