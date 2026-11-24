use crate::commands::{Connection, Credentials};
use clap::{Parser, Subcommand};

use crate::args::escapade::SubEscapades;
use crate::args::heartbeat::SubHeartbeat;

pub mod escapade;
pub mod heartbeat;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Input>,
}

#[derive(Subcommand)]
pub enum Input {
    Server(Connection),
    /// Signup
    Register(Credentials),
    /// Login
    Login(Credentials),
    /// Logout
    Logout {
        #[command(subcommand)]
        command: Option<SubLogout>,
    },
    /// Show config status
    Status,
    /// Create escapade
    Escapade {
        #[command(subcommand)]
        command: SubEscapades,
    },
    /// Post
    Heartbeat {
        #[command(subcommand)]
        command: SubHeartbeat,
    },
}

#[derive(Subcommand)]
pub enum SubLogout {
    /// Delete all sessions (including on other devices)
    All,
}
