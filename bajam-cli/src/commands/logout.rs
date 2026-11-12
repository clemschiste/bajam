use crate::errors::CliError;
use crate::commands::{load_session_config, create_session_config};
use crate::commands::Config;

pub async fn logout() -> Result<(), CliError> {
        let config = load_session_config()?;

       // Set token value to None as it is now to valid anymore
        let new_config = Config {
            server_info: config.server_info,
            connected_user: None,
        };
        create_session_config(new_config)?;

        println!("Successfully logged out.");

        Ok(())
        
}
