use crate::commands::{create_session_config, Config, Connection, CliError};

pub async fn set_server(connection: Connection) -> Result<(), CliError> {
    let config = Config {
        connection,
        token: None,
    };
    create_session_config(&config)?;

    Ok(())
}
