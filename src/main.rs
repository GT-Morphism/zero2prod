use tracing_subscriber::EnvFilter;
use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| {
                    EnvFilter::try_new("zero2prod=trace,tower_http=trace,axum::rejection=trace")
                })
                .unwrap(),
        )
        .init();

    run().await.expect("Failed running application.");
}
