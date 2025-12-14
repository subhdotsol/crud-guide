use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // Create a simple router (no routes yet)
    let app = Router::new();

    // Bind to localhost on port 3000
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind to address");

    println!("🚀 Server listening on http://127.0.0.1:3000");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
