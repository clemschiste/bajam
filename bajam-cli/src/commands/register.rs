use crate::commands::{Credentials, create_session_config, get_auth_cookie, load_session_config, register_request, CliError};

/// based on credentials create a new user (post on user/new) and refresh config to be logged in directly
pub async fn register_and_login(credentials: &Credentials) -> Result<(), CliError> {
    let client = reqwest::Client::new();
    // signup
    register_request(client, credentials).await?;
    // login request to get token
    let token = get_auth_cookie(credentials).await?;

    let mut config = load_session_config()?;
    config.token = Some(token);
    println!("Welcome, {} !", credentials.username);
    
    create_session_config(&config)?;
    println!("Session configured");

    Ok(())
}
