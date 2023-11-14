use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn health_check() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "ok",
        "message": "success"
    });

    (StatusCode::OK, Json(json_response))
}
