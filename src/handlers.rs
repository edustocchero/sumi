use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bcrypt::DEFAULT_COST;

use crate::{contracts, models::User, AppState};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<contracts::CreateUser>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let passw_hash = bcrypt::hash(body.password, DEFAULT_COST).unwrap();

    let sql = "INSERT INTO users (username, email, passw) VALUES ($1, $2, $3) RETURNING *";

    let query: Result<User, _> = sqlx::query_as(sql)
        .bind(&body.username)
        .bind(&body.email)
        .bind(&passw_hash)
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
