use crate::commands::{Config, Connection, Credentials, create_session_config, get_auth_cookie, register_request};

/// based on credentials create a new user (post on user/new) and refresh config to be logged in directly
pub async fn register_and_login(connection: &Connection, credentials: &Credentials) -> anyhow::Result<()> {
    let client = reqwest::Client::new();

    // signup
    register_request(client, credentials, connection).await?;

    // login request to get token
    let token = get_auth_cookie(credentials, connection).await?;

    // create config file
    let config = Config {
      connection: connection.clone(),
      token
    };
    create_session_config(&config)?;

    Ok(())
}
