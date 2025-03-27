use axum::{
    Router,
    routing::{get, post},
};

use crate::routes::{health_check, subscribe};

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run() -> Result<(), std::io::Error> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await?;

    axum::serve(listener, app()).await?;

    Ok(())
}
