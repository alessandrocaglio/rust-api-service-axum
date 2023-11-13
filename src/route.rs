use crate::handler::health_check;

use axum::{
    routing::get,
    Router, 
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello, world!" }))
        .route("/health", get(health_check))
        
}