use serde::Deserialize;

use axum::{Form, http::StatusCode};

#[derive(Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(Form(form): Form<FormData>) -> StatusCode {
    println!("name {}, email {}", form.name, form.email);
    StatusCode::OK
}
