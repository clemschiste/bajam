use crate::commands::{create_session_config, Config, Connection};
use crate::errors::CliError;

pub async fn set_server(new_connection: Connection) -> Result<(), CliError> {
    let config = Config {
        server_info: new_connection,
        connected_user: None,
    };
    create_session_config(config)?;
    println!("Server setting adjusted in config.toml.");

    Ok(())
}
