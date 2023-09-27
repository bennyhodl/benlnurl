use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BenlnurlError {
    pub status: String,
    pub reason: String,
}

pub fn build_error(reason: &str) -> Response {
    (
        StatusCode::NOT_FOUND,
        Json(BenlnurlError {
            status: "ERROR".to_string(),
            reason: reason.to_string(),
        }),
    )
        .into_response()
}
