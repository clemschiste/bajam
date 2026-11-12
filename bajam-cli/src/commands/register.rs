use std::collections::HashMap;

use reqwest::StatusCode;

use crate::commands::{Credentials, load_session_config};
use crate::errors::CliError;

pub async fn register(credentials: &Credentials) -> Result<(), CliError> {
    let client = reqwest::Client::new();
    let connection = load_session_config()?.server_info;

    let mut body_json = HashMap::new();
    body_json.insert("username", credentials.username.clone());
    body_json.insert("password", credentials.password.clone());
    let body = client
        .post(format!("http://{}:{}/user/new", connection.adress, connection.port))
        .json(&body_json)
        .send()
        .await?;

    match body.status() {
        StatusCode::NO_CONTENT => println!("User successfully created"),
        status => {
            let body_text = body.text().await?;
            return Err(CliError::HttpError(status, body_text));
        }
    }

    Ok(())
}
