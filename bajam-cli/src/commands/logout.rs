use reqwest::{StatusCode, header};

use crate::errors::CliError;
use crate::commands::{load_session_config, create_session_config};
use crate::commands::Config;

pub async fn logout() -> Result<(), CliError> {
        let config = load_session_config()?;
        let client = reqwest::Client::new();

        let user = match &config.connected_user {
            Some(user) => user,
            None => return Err(CliError::UncompleteConfig)
        };

        let response = client
            .post(format!("http://{}:{}/api/logout", config.server_info.address, config.server_info.port))
            .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
            .send()
        .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(CliError::Auth("Wrong credentials".to_string()));
        }

       // Set token value to None as it is now to valid anymore
        let new_config = Config {
            server_info: config.server_info,
            connected_user: None,
        };
        create_session_config(new_config)?;

        println!("Successfully logged out.");

        Ok(())
        
}

pub async fn logout_all_user_sessions() -> Result<(), CliError> {
        let config = load_session_config()?;
        let client = reqwest::Client::new();

        let user = match &config.connected_user {
            Some(user) => user,
            None => return Err(CliError::UncompleteConfig)
        };

        let response = client
            .post(format!("http://{}:{}/api/logout/all", config.server_info.address, config.server_info.port))
            .header(header::AUTHORIZATION, format!("Bearer {}", user.token))
            .send()
        .await?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(CliError::Auth("Wrong credentials".to_string()));
        }

       // Set token value to None as it is now to valid anymore
        let new_config = Config {
            server_info: config.server_info,
            connected_user: None,
        };
        create_session_config(new_config)?;

        println!("You are now logged out from every sessions.");

        Ok(())
        
}
