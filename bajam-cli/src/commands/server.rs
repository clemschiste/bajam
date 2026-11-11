use crate::commands::{create_session_config, Config, Connection, CliError};

pub async fn set_server(connection: Connection) -> Result<(), CliError> {
    let config = Config {
        server_info: connection,
        connected_user: None,
    };
    create_session_config(&config)?;
    println!("Server setting adjusted in config.toml.");

    Ok(())
}
