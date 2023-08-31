use serde::Serialize;
use sqlx::{postgres::PgRow, FromRow, Row};

#[derive(Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let username = row.try_get("username")?;
        let email = row.try_get("email")?;
        let password = row.try_get("passw")?;

        Ok(User {
            username,
            email,
            password,
        })
    }
}
