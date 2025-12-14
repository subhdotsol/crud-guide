// Health check handler

use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

/// Health check endpoint
/// Returns a simple JSON response to confirm the server is running
pub async fn health() -> (StatusCode, Json<Value>) {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is healthy"
        })),
    )
}
