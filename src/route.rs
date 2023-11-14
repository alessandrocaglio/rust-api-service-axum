use crate::handler::health_check;

use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health", get(health_check))
}

#[cfg(test)]
mod tests {
    use super::*;

    use axum::http;
    use hyper::{Body, Request, StatusCode};
    use tower::util::ServiceExt;
    #[tokio::test]
    async fn test_handle_health_check() {
        let app = create_router();

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::GET)
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
