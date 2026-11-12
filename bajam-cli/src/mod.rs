use reqwest::{Client};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use clap::{Args};
use reqwest::StatusCode;

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

// Create the config file if it does not exists. Must first get login response from server.
pub fn create_session_config(config: &Config) -> Result<(), CliError> {
    let toml = toml::to_string_pretty(config)?;
    fs::write("config.toml", toml)?;
    Ok(())
}

// The session already exists. Does not check if it is still valid from the server perspective
pub fn load_session_config() -> Result<Config, CliError> {
    let config_toml = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_toml)?;

    Ok(config)
}

pub async fn get_auth_token(credentials: &Credentials) -> Result<String, CliError> {
    let connection = load_session_config()?.server_info;
    
    let client = reqwest::Client::new();
    let mut body_json = HashMap::new();
    body_json.insert("username", credentials.username.clone());
    body_json.insert("password", credentials.password.clone());
    let body = client
        .post(format!("http://{}:{}/user/login", connection.adress, connection.port))
        .json(&body_json)
        .send()
        .await?;

    let login_response: LoginResponse = serde_json::from_str(body.text().await?.as_str())?;
    Ok(login_response.token)
}

// Logout will simply be a clearance of the token value (still valid for other sessions)
pub async fn logout() -> Result<(), CliError> {
        let config = load_session_config()?;

       // Set token value to None as it is now to valid anymore
        let new_config = Config {
            server_info: config.server_info,
            connected_user: None,
        };
        create_session_config(&new_config)?;

        println!("Successfully logged out.");

        Ok(())
        
}

// Status reads the config file and display server and user status
pub async fn status() -> Result<(), CliError> {
        let config = load_session_config()?;
        let current_user = match config.connected_user {
            Some(user) => user.username,
            None => "None".to_string(),
        };
        println!("User: {}.\nServer: http://{}:{}.",
            current_user,
            config.server_info.adress,
            config.server_info.port);

        Ok(())
        
}

pub async fn register_request(client: Client, credentials: &Credentials) -> Result<(), CliError> {
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

// A mettre dans lib commune...
#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
