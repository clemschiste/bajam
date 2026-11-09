use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};
use clap::{Args};
use reqwest::StatusCode;

pub mod register;
pub mod server;

#[derive(Debug)]
pub enum CliError {
    Io(std::io::Error),
    Toml(toml::de::Error),
    TomlSer(toml::ser::Error),
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
    MissingConfig,
    HttpError(reqwest::StatusCode, String),
    Auth(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::Io(e) => write!(f, "Erreur d'entrée/sortie: {e}"),
            CliError::Toml(e) => write!(f, "Erreur de lecture du fichier de configuration (TOML): {e}"),
            CliError::TomlSer(e) => write!(f, "Erreur d'écriture du fichier de configuration (TOML): {e}"),
            CliError::Reqwest(e) => write!(f, "Erreur réseau: {e}"),
            CliError::SerdeJson(e) => write!(f, "Erreur de désérialisation JSON: {e}"),
            CliError::MissingConfig => write!(
                f,
                "Aucun serveur configuré. Utilisez `bajam server --adresse <ADDR> --port <PORT>` d'abord."
            ),
            CliError::HttpError(status, body) => write!(f, "Erreur HTTP {status}: {body}"),
            CliError::Auth(msg) => write!(f, "Erreur d'authentification: {msg}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

impl From<toml::de::Error> for CliError {
    fn from(e: toml::de::Error) -> Self {
        CliError::Toml(e)
    }
}

impl From<toml::ser::Error> for CliError {
    fn from(e: toml::ser::Error) -> Self {
        CliError::TomlSer(e)
    }
}

impl From<reqwest::Error> for CliError {
    fn from(e: reqwest::Error) -> Self {
        CliError::Reqwest(e)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(e: serde_json::Error) -> Self {
        CliError::SerdeJson(e)
    }
}

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
    pub token: Option<String>,
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

pub async fn get_auth_cookie(credentials: &Credentials) -> Result<String, CliError> {
    let connection = load_session_config()?.connection;
    
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

pub async fn register_request(client: Client, credentials: &Credentials) -> Result<(), CliError> {
    let connection = load_session_config()?.connection;

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
