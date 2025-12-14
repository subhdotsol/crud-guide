// Route definitions

use axum::{routing::get, Router};
use sqlx::PgPool;

use crate::handlers;

/// Create and configure all application routes
pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .route("/health", get(handlers::health::health))
}
