use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{CorsLayer, AllowOrigin};

#[tokio::main]
async fn main() {
    // initialize tracing

    let cors = CorsLayer::new().allow_origin(any);

    let app = Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .layer(cors);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));


    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");


}
