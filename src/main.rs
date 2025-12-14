use tokio::net::TcpListener;

use rust_crud::routes;

#[tokio::main]
async fn main() {
    // Create the application router with all routes
    let app = routes::create_routes();

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