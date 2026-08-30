use crate::graphql_handlers::{graphiql_handler, graphql_handler};
use crate::state::AppState;
use axum::{
    Router,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};

pub fn build_router(state: AppState) -> Router {
    let graphql_routes = if state.config.graphql.graphiql {
        Router::new().route("/graphql", get(graphiql_handler).post(graphql_handler))
    } else {
        Router::new().route("/graphql", post(graphql_handler))
    };

    Router::new()
        .route("/healthz", get(health))
        .route("/readyz", get(readiness))
        .merge(graphql_routes)
        .with_state(state)
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}

async fn readiness() -> impl IntoResponse {
    // TODO: need to check db::health_check and return Result<IntoResponse, StatusCode>
    StatusCode::OK
}
