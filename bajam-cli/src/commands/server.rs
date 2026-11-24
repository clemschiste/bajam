use crate::commands::logout::logout;
use crate::commands::{Config, Connection, create_session_config, load_session_config};
use crate::errors::CliError;

pub async fn set_server(new_connection: Connection) -> Result<(), CliError> {
    // delete session if the config has a connected_user
    let config_opt = load_session_config().ok();

    if let Some(config) = config_opt && config.connected_user.is_some() {
            logout().await?;
    }
    
    let config = Config {
        server_info: new_connection,
        connected_user: None,
    };
    create_session_config(config)?;
    println!("Server setting adjusted. You can try to logout to the new server.");

    Ok(())
}
