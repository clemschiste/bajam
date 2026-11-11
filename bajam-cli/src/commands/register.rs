use crate::commands::{Credentials, create_session_config, get_auth_token, load_session_config, register_request, CliError};

/// based on credentials create a new user (post on user/new) and refresh config to be logged in directly
pub async fn register_and_login(credentials: &Credentials) -> Result<(), CliError> {
    let client = reqwest::Client::new();
    // signup
    register_request(client, credentials).await?;
    // login request to get token
    let token = get_auth_token(credentials).await?;

    let mut config = load_session_config()?;
    config.token = Some(token);
    println!("Welcome, {} !", credentials.username);
    
    create_session_config(&config)?;
    println!("Session configured");

    Ok(())
}

// login récupère le auth cookie du serveur et le remplace par l'existant du fichier config
// when using login you reload the session token, it should not be the case (for now)
pub async fn login(credentials: &Credentials) -> Result<(), CliError> {
    let token = get_auth_token(credentials).await?;
    let mut config = load_session_config()?;

    config.token = Some(token);
    println!("You are now logged in, {} !", credentials.username);
    
    create_session_config(&config)?;
    println!("Session configured");

    Ok(())
}

