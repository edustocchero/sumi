use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{handlers::*, Env};

pub fn router(state: Arc<Env>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
        .with_state(state)
}
