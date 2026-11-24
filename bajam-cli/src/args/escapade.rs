use crate::errors::CliError;
use clap::Subcommand;
use std::str::FromStr;

#[derive(Subcommand)]
pub enum SubEscapades {
    /// List available escapades
    List,
    /// Create an escapade (you will be admin)
    Create(EscapadeArgs),
    /// Delete an escapade (you must be admin of the escapade)
    Delete,
    /// Shows heartbeat within escapade
    Show,
}
#[derive(clap::Args)]
pub struct EscapadeArgs {
    pub name: String,
}

impl EscapadeArgs {
    /// Checks if the name is not empty
    pub fn valid_name(&self) -> Result<bool, CliError> {
        if self.name.is_empty() {
            Ok(true)
        } else {
            Err(CliError::InvalidInput(
                "Escapade name cannot be empty.".to_string(),
            ))
        }
    }
}

#[derive(Clone)]
pub enum EscInput {
    Id(i64),
    Name(String),
}

// Parse i64 (id) ou String (name)
impl FromStr for EscInput {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(id) => Ok(EscInput::Id(id)),
            Err(_) => Ok(EscInput::Name(s.to_string())),
        }
    }
}
