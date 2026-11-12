use crate::errors::CliError;
use crate::commands::load_session_config;

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
