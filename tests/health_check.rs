use std::net::SocketAddr;

use zero2prod::startup::app;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

async fn spawn_app() -> String {
    let address = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    tokio::spawn(async move {
        axum::serve(listener, app()).await.unwrap();
    });

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let body = "name=Don%20Giovanni&email=don%40giovanni.com";

    let response = client
        .post(format!("{}/subscriptions", address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=don%20giovanni", "missing the email"),
        ("email=don%40giovanni.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 422 Unprcessable Content when the payload was {}.",
            error_message
        );
    }
}
