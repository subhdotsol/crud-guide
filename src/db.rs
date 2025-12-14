// Database connection module

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

/// Create a PostgreSQL connection pool
/// 
/// This function:
/// - Reads DATABASE_URL from environment variables
/// - Creates a connection pool with max 5 connections
/// - Sets connection timeout to 30 seconds
/// - Returns the pool for use throughout the app
pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&database_url)
        .await?;

    println!("✅ Database connection pool created");

    Ok(pool)
}
