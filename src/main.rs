mod handler;
mod route;

use std::time::Duration;

use axum::{
    body::{Body, BoxBody},
    http::{Request, Response},
    Router,
};
use tower_http::{
    classify::ServerErrorsFailureClass,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::Span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let http_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Unable to parse env variable PORT");

    let app = app();

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], http_port));

    tracing::warn!("Listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn app() -> Router {
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(|_request: &Request<Body>| {
            let request_id = Uuid::new_v4().to_string();
            tracing::info_span!("http-request", %request_id)
        })
        .on_request(|request: &Request<Body>, _span: &Span| {
            tracing::trace!("request: {} {}", request.method(), request.uri().path())
        })
        .on_response(
            |response: &Response<BoxBody>, latency: Duration, _span: &Span| {
                tracing::trace!("response: {} {:?}", response.status(), latency)
            },
        )
        .on_failure(
            |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                tracing::error!("error: {}", error)
            },
        );

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api_service_axum=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    route::create_router().layer(cors).layer(tracing_layer)
}
