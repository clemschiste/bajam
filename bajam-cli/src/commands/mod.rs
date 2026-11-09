use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use clap::{Args};
use reqwest::StatusCode;

pub mod signup;

#[derive(Args, Clone)]
pub struct Credentials {
    #[arg(short, long)]
    pub username: String,
    #[arg(short, long)]
    pub password: String,
}

#[derive(Args, Serialize, Deserialize, Clone)]
pub struct Connection {
    #[arg(short, long)]
    pub adress: String,
    #[arg(short, long)]
    pub port: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub connection: Connection,
    pub token: String,
}

// Create the config file if it does not exists. Must first get login response from server.
pub fn create_session_config(config: &Config) -> anyhow::Result<()> {
    let toml = toml::to_string_pretty(config)?;
    fs::write("config.toml", toml)?;
    Ok(())
}

// The session already exists. Does not check if it is still valid from the server perspective
pub fn load_session_config() -> anyhow::Result<Config> {
    let config_toml = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    Ok(config)
}

pub async fn get_auth_cookie(credentials: &Credentials, connection: &Connection) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let mut body_json = HashMap::new();
    body_json.insert("username", credentials.username.clone());
    body_json.insert("password", credentials.password.clone());
    let body = client
        .post(format!("{}:{}/user/login", connection.adress, connection.port))
        .json(&body_json)
        .send()
        .await?;

    let login_response: LoginResponse = serde_json::from_str(body.text().await?.as_str())?;
    Ok(login_response.token)
}

pub async fn register_request(client: Client, credentials: &Credentials, connection: &Connection) -> anyhow::Result<()> {
    let mut body_json = HashMap::new();
    body_json.insert("username", credentials.username.clone());
    body_json.insert("password", credentials.password.clone());
    let body = client
        .post(format!("{}:{}/user/new", connection.adress, connection.port))
        .json(&body_json)
        .send()
        .await?;

    match body.status() {
      StatusCode::NO_CONTENT => println!("User successfully created"),
      _ => eprint!("User not created, error from server")
    }

    Ok(())
}

// A mettre dans lib commune...
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
