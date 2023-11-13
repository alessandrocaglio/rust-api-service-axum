mod handler;
mod route;

use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let http_port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Unable to parse env variable PORT");

    let cors = CorsLayer::new().allow_origin(Any);

    let app = route::create_router().layer(cors);

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], http_port));

    println!("Listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
