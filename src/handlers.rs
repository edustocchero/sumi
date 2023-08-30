use axum::{extract::State, Json, response::IntoResponse, http::StatusCode};
use bcrypt::DEFAULT_COST;

use crate::{AppState, contracts, models::User};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<contracts::CreateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let passw_hash = bcrypt::hash(body.password, DEFAULT_COST).unwrap();
    let query = sqlx::query_as!(
        User,
        "INSERT INTO users (username, email, passw) VALUES ($1, $2, $3) RETURNING *",
        body.username,
        body.email,
        passw_hash,
    )
    .fetch_one(&state.db)
    .await;

    match query {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            eprintln!("{:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
