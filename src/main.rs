use zero2prod::startup::run;

#[tokio::main]
async fn main() {
    run().await.expect("Failed running application.");
}
