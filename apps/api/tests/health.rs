mod common;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use api::router::build_router;
use api::state::AppState;

#[tokio::test]
async fn health_returns_ok() {
    // Arrange
    let state = AppState::test();
    let app = build_router(state);
    let request = Request::builder()
        .uri("/healthz")
        .body(Body::empty())
        .unwrap();

    // Act
    let response = app.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
