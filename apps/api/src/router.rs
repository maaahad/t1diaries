use crate::graphql_handlers::{graphiql_handler, graphql_handler};
use crate::state::AppState;
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/graphql", get(graphiql_handler).post(graphql_handler))
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
