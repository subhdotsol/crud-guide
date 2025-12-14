// Health check handler

use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};
use sqlx::PgPool;

/// Health check endpoint
/// Returns server status and database connection status
pub async fn health(State(pool): State<PgPool>) -> (StatusCode, Json<Value>) {
    // Test database connection with a simple query
    let db_status = match sqlx::query("SELECT 1")
        .fetch_one(&pool)
        .await
    {
        Ok(_) => "connected",
        Err(_) => "disconnected",
    };

    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "database": db_status
        })),
    )
}
