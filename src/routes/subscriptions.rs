use axum::{Form, extract::State, http::StatusCode};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::configuration::AppState;

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(State(state): State<AppState>, Form(form): Form<FormData>) -> StatusCode {
    println!("name {}, email {}", form.name, form.email);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(&state.db_pool)
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    };

    StatusCode::OK
}
