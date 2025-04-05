use axum::{
    Router,
    routing::{get, post},
};
use sqlx::{Connection, PgConnection};
use std::sync::Arc;

use crate::configuration::get_configuration;
use crate::routes::{health_check, subscribe};

pub fn app(connection: Arc<PgConnection>) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(connection)
}

pub async fn run() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], configuration.application_port));
    let listener = tokio::net::TcpListener::bind(&address).await?;

    axum::serve(listener, app(connection.into())).await?;

    Ok(())
}
