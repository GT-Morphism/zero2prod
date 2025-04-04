use axum::http::StatusCode;

// remove me
pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
