use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub passw: String,
}
