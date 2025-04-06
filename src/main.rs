use tracing::info;
use zero2prod::startup::run;
use zero2prod::telemetry::init_telemetry;

#[tokio::main]
async fn main() {
    init_telemetry();

    info!("Running application");
    run().await.expect("Failed running application.");
}
