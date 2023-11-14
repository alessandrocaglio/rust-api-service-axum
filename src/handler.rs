use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use hyper::Client;
use hyper_tls::HttpsConnector;

pub async fn health_check() -> impl IntoResponse {
    let json_response = serde_json::json!({
        "status": "ok",
        "message": "success"
    });

    (StatusCode::OK, Json(json_response))
}

pub async fn get_todos(Path(id): Path<u32>) -> impl IntoResponse {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let uri = format!("https://jsonplaceholder.typicode.com/todos/{}", id)
        .parse()
        .unwrap();

    let response = client.get(uri).await.unwrap();
    let status_code = response.status();

    let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();

    (status_code, body_bytes)
}
