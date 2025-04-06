use axum::{
    Router,
    http::Request,
    routing::{get, post},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{
    LatencyUnit, ServiceBuilderExt,
    request_id::MakeRequestUuid,
    trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

use crate::configuration::{AppState, get_configuration};
use crate::routes::{health_check, subscribe};

pub fn app(state: AppState) -> Router {
    let svc = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let name = format!("{} {}", request.method(), request.uri());

                    tracing::info_span!(
                        "request",
                        otel.name = name,
                        method = %request.method(),
                        uri = %request.uri(),
                        headers = ?request.headers(),
                        version = ?request.version(),
                    )
                })
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(LatencyUnit::Micros)
                        .include_headers(true),
                ),
        )
        .propagate_x_request_id();

    Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(svc)
        .with_state(state)
}

pub async fn run() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
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
