use crate::api::{status, login};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/api/v2/status", get(status))
        .route("/api/v2/login", post(login))
}
