use axum::{
    Router,
    routing::{get, post},
};

use crate::configuration::get_configuration;
use crate::routes::{health_check, subscribe};

pub fn app() -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
}

pub async fn run() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], configuration.application_port));
    let listener = tokio::net::TcpListener::bind(&address).await?;

    axum::serve(listener, app()).await?;

    Ok(())
}
