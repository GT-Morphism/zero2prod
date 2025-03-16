use std::net::SocketAddr;

use axum::{
    Form, Router,
    http::StatusCode,
    routing::{get, post},
};

use serde::Deserialize;

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
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(Form(form): Form<FormData>) -> StatusCode {
    println!("name {}, email {}", form.name, form.email);
    StatusCode::OK
}
