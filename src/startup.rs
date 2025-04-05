use axum::{
    Router,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::configuration::{AppState, get_configuration};
use crate::routes::{health_check, subscribe};

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state)
}

pub async fn run() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = std::net::SocketAddr::from(([0, 0, 0, 0], configuration.application_port));
    let listener = tokio::net::TcpListener::bind(&address).await?;
    let state = AppState {
        db_pool: connection_pool,
    };

    axum::serve(listener, app(state)).await?;

    Ok(())
}
