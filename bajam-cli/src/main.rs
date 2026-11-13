use clap::{Parser, Subcommand};
use commands::{Connection, Credentials};

use crate::commands::server::set_server;
use crate::commands::register::*;
use crate::commands::login::*;
use crate::commands::logout::*;
use crate::commands::status::*;
use crate::commands::escapades::*;

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
    Heartbeat,
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

#[derive(clap::Args)]
pub struct EscapadeArgs {
    pub name: String,
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
                SubEscapades::Create(escapade) => (create_escapade(escapade.name)).await?,
                SubEscapades::Delete => (),
            }
        },
        None => println!("No argument provided."),
        _ => panic!()
    }

    Ok(())
}
