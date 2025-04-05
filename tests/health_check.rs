use std::net::SocketAddr;

use sqlx::{Connection, PgConnection};
use zero2prod::{configuration::get_configuration, startup::app};

pub struct TestApp {
    pub address: String,
    pub db_connection: PgConnection,
}

async fn spawn_app() -> TestApp {
    let address = SocketAddr::from(([0, 0, 0, 0], 0));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    let port = listener.local_addr().unwrap().port();

    let configuration = get_configuration().expect("Failed to read configuration.");
    let app_connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let test_connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    tokio::spawn(async move {
        axum::serve(listener, app(app_connection.into()))
            .await
            .unwrap();
    });

    TestApp {
        address: format!("http://127.0.0.1:{}", port),
        db_connection: test_connection,
    }
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let mut app = spawn_app().await;

    let client = reqwest::Client::new();
    let body = "name=Don%20Giovanni&email=don%40giovanni.com";

    let response = client
        .post(format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut app.db_connection)
        .await
        .expect("Failed to fetch saved subscriptions");
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=don%20giovanni", "missing the email"),
        ("email=don%40giovanni.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{}/subscriptions", &app.address))
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
