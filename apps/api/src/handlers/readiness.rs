use axum::{http::StatusCode, response::IntoResponse};

pub async fn readiness() -> impl IntoResponse {
    // TODO: need to check db::health_check and return Result<IntoResponse, StatusCode>
    StatusCode::OK
}
