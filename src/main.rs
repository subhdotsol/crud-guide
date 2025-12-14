use tokio::net::TcpListener;

use rust_crud::{db, routes};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Create database connection pool
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");

    // Create the application router with database pool as shared state
    let app = routes::create_routes().with_state(pool);

    // Bind to localhost on port 3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server listening on http://127.0.0.1:3000");
    println!("📍 Health check: http://127.0.0.1:3000/health");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}