use crate::state::AppState;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
