use std::net::SocketAddr;

#[tokio::test]
async fn health_check_works() {
    spawn_app().await;

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:3000/health_check")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() {
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("Failed to parse PORT");

    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    tokio::spawn(async move {
        axum::serve(listener, zero2prod::app()).await.unwrap();
    });
}
