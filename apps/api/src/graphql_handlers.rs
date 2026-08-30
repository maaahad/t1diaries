use crate::state::AppState;
use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn graphql_handler(
    State(state): State<AppState>,
    request: GraphQLRequest,
) -> GraphQLResponse {
    state.schema.execute(request.into_inner()).await.into()
}

pub async fn graphiql_handler() -> Html<String> {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

pub async fn health() -> impl IntoResponse {
    StatusCode::OK
}
