// Route definitions

use axum::{routing::get, Router};

use crate::handlers;

/// Create and configure all application routes
pub fn create_routes() -> Router {
    Router::new()
        .route("/health", get(handlers::health::health))
}
