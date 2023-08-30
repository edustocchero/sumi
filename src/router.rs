use axum::{routing::get, Router};

use crate::handlers::*;

pub fn router() -> Router {
    Router::new().route("/", get(root))
}
