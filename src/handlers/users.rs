// Users handler - CRUD operations for users

use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::{CreateUser, User};

/// Create a new user
/// POST /users
pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    // Insert user into database
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (name, email, age)
        VALUES ($1, $2, $3)
        RETURNING id, name, email, age, created_at, updated_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(payload.age)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        eprintln!("Database error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to create user: {}", e),
        )
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}
