use axum::http::StatusCode;
use tracing::info;

#[tracing::instrument(name = "Check if application is running")]
pub async fn health_check() -> StatusCode {
    info!("App is running");
    StatusCode::OK
}
