use crate::graphql_handlers::{graphiql_handler, graphql_handler, health};
use crate::state::AppState;
use axum::{
    Router,
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
        .merge(graphql_routes)
        .with_state(state)
}
