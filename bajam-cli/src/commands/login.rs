use crate::{commands::Credentials, errors::CliError};
use crate::commands::{get_auth_token, load_session_config, create_session_config_with_user};
use crate::commands::CurrentUser;

// login récupère le auth cookie du serveur et le remplace par l'existant du fichier config
// when using login you reload the session token, it should not be the case (for now)
pub async fn login(credentials: &Credentials) -> Result<(), CliError> {
    let token = get_auth_token(credentials).await?;
    let mut config = load_session_config()?;

    config.connected_user = Some(
        CurrentUser {
            username: credentials.username.clone(),
            token,
        }
    );
    println!("You are now logged in, {} !", credentials.username);
    
    create_session_config_with_user(config)?;
    println!("Session configured");

    Ok(())
}

