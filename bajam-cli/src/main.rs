use clap::{Parser, Subcommand};
use commands::{Connection, Credentials};
use std::str::FromStr;

use crate::commands::heartbeat::create_heartbeat;
use crate::commands::server::set_server;
use crate::commands::register::*;
use crate::commands::login::*;
use crate::commands::logout::*;
use crate::commands::status::*;
use crate::commands::escapades::*;
use crate::errors::CliError;

mod commands;
mod errors;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Input>,
}

#[derive(Subcommand)]
enum Input {
    Server(Connection),
    /// Signup
    Register(Credentials),
    /// Login
    Login(Credentials),
    // Logout
    Logout,
    // Show config status
    Status,
    // Create escapade
    Escapade {
        #[command(subcommand)]
        command: SubEscapades,
    },
    // Post
    Heartbeat {
        #[command(subcommand)]
        command: SubHeartbeat,
    },
}

#[derive(Subcommand)]
enum SubEscapades {
    // List available escapades
    List,
    // Create an escapade (you will be admin)
    Create(EscapadeArgs),
    // Delete an escapade (you must be admin of the escapade)
    Delete,
}

#[derive(Subcommand)]
enum SubHeartbeat {
    Create(HeartbeatArgs),
    Show(HeartbeatArgs),
}

#[derive(clap::Args)]
pub struct EscapadeArgs {
    pub name: String,
}

impl EscapadeArgs {
    /// Checks if the name is not empty
    fn valid_name(&self) -> Result<bool, CliError> {
        if self.name.is_empty() {
            Ok(true)
        } else {
            Err(CliError::InvalidInput("Escapade name cannot be empty.".to_string()))                    
        }
    }
}

#[derive(clap::Args)]
pub struct HeartbeatArgs {
    pub escinput: EscInput,
    pub latitude: f64,
    pub longitude: f64,
    pub description: Option<String>,
}

impl HeartbeatArgs {
    /// Checks if the latitude and longitude are within expected range of -90.0 - 90.0
    fn valid_latitude(&self) -> Result<bool, CliError> {
        if self.latitude <= 90.0
            && self.longitude <= 90.0
            && self.latitude >= -90.0
            && self.longitude >= -90.0
        {
            Ok(true)
        } else {
            Err(CliError::InvalidInput("Lattiude and longitude must be between -90.0 and 90.0".to_string()))
        }
    }

    /// Checks if the name is not empty
    fn valid_name(&self) -> Result<bool, CliError> {
        match &self.escinput {
            EscInput::Name(value) => {
                if !value.is_empty() {
                    Ok(true)
                } else {
                    Err(CliError::InvalidInput("Escapade name cannot be empty.".to_string()))                    
                }
            }
            _ => Ok(true)
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Input::Server(connection)) => set_server(connection).await?,
        Some(Input::Register(credentials)) => register(&credentials).await?,
        Some(Input::Login(credentials)) => login(&credentials).await?,
        Some(Input::Logout) => logout().await?,
        Some(Input::Status) => status().await?,
        Some(Input::Escapade{ command: action }) => {
            match action {
                SubEscapades::List => list_escapades().await?,
                SubEscapades::Create(args) => {
                    if args.valid_name()? {
                        (create_escapade(args.name)).await?
                    }
                }
                SubEscapades::Delete => (),
            }
        },
        Some(Input::Heartbeat{ command: action }) => {
            match action {
                SubHeartbeat::Create(args) => {
                    if args.valid_latitude()? && args.valid_name()? {
                        create_heartbeat(args.escinput, args.latitude, args.longitude, args.description).await?;
                    }
                }
                SubHeartbeat::Show(_) => (),
            }
        },
        None => println!("No argument provided."),
    }

    Ok(())
}
