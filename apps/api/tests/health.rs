mod common;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};

use tower::ServiceExt;

use crate::common::app::TestApp;

#[tokio::test]
async fn health_endpoint_returns_ok() {
    // Arrange
    let app = TestApp::new().build();
    let request = Request::builder()
        .uri("/healthz")
        .body(Body::empty())
        .unwrap();

    // Act
    let response = app.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
