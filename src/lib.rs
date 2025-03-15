use std::net::SocketAddr;

use axum::{Router, http::StatusCode, routing::get};

pub async fn run() -> Result<(), std::io::Error> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await?;

    axum::serve(listener, app()).await?;

    Ok(())
}

pub fn app() -> Router {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}
