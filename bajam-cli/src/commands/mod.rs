use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use clap::{Args};
use bajam_common::models::session::LoginResponse;

use crate::errors::CliError;

pub mod register;
pub mod server;
pub mod post;
pub mod logout;
pub mod status;
pub mod login;
pub mod escapades;

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CurrentUser {
    username: String,
    token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub server_info: Connection,
    pub connected_user: Option<CurrentUser>,
}

impl Config {
    /// Check if it is valid for authentication attempt (connected user is defined)
    pub fn has_user(&self) -> bool {
        self.connected_user.is_some()
    }
}

// Create the config file if it does not exists. Config must be valid (including token).
pub fn create_session_config_with_user(config: Config) -> Result<(), CliError> {
    if !&config.has_user() {
        return Err(CliError::UncompleteConfig);
    }

    let toml = toml::to_string_pretty(&config)?;
    fs::write("config.toml", toml)?;
    Ok(())
}

// Create the config file if it does not exists. User might be set to None here.
pub fn create_session_config(config: Config) -> Result<(), CliError> {
    let toml = toml::to_string_pretty(&config)?;
    fs::write("config.toml", toml)?;
    Ok(())
}

// The session already exists. Does not check if it is still valid from the server perspective
pub fn load_session_config() -> Result<Config, CliError> {
    let config_toml = fs::read_to_string("config.toml")
        .map_err(|e| CliError::MissingConfig(
            format!("{e}\nConfig file not found: server not defined. Try 'server' command first.")
        ))?;
    let config: Config = toml::from_str(&config_toml)?;

    Ok(config)
}

pub async fn get_auth_token(credentials: &Credentials) -> Result<String, CliError> {
    let connection = load_session_config()?.server_info;
    
    let client = reqwest::Client::new();
    let mut body_json = HashMap::new();
    body_json.insert("username", credentials.username.clone());
    body_json.insert("password", credentials.password.clone());
    let response = client
        .post(format!("http://{}:{}/user/login", connection.adress, connection.port))
        .json(&body_json)
        .send()
    .await?;

    if response.status() == StatusCode::UNAUTHORIZED {
        return Err(CliError::Auth("Wrong credentials".to_string()));
    }

    let response_body: LoginResponse = serde_json::from_str(response.text().await?.as_str())?;

    Ok(response_body.token)
}
