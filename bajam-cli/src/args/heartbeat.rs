use crate::args::escapade::EscInput;
use crate::errors::CliError;
use clap::Subcommand;

#[derive(Subcommand)]
pub enum SubHeartbeat {
    /// Create a new heartbeat
    Create(HeartbeatArgs),
    Show(HeartbeatArgs),
    /// List heartbeats of an escapade
    List(HeartbeatListArgs),
}
#[derive(clap::Args)]
pub struct HeartbeatArgs {
    pub escapade: EscInput,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

#[derive(clap::Args)]
pub struct HeartbeatListArgs {
    pub escapade: EscInput,
}

impl HeartbeatArgs {
    /// Checks if the latitude and longitude are within expected range of -90.0 - 90.0
    pub fn valid_latitude(&self) -> Result<bool, CliError> {
        if self.latitude <= 180.0
            && self.longitude <= 180.0
            && self.latitude >= -180.0
            && self.longitude >= -180.0
        {
            Ok(true)
        } else {
            Err(CliError::InvalidInput(
                "Lattiude and longitude must be between -90.0 and 90.0".to_string(),
            ))
        }
    }

    /// Checks if the name is not empty
    pub fn valid_name(&self) -> Result<bool, CliError> {
        match &self.escapade {
            EscInput::Name(value) => {
                if !value.is_empty() {
                    Ok(true)
                } else {
                    Err(CliError::InvalidInput(
                        "Escapade name cannot be empty.".to_string(),
                    ))
                }
            }
            _ => Ok(true),
        }
    }
}
