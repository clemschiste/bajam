use clap::{Parser, Subcommand};
use commands::{Connection, Credentials};

use crate::commands::server::set_server;
use crate::commands::register::*;
use crate::commands::login::*;
use crate::commands::logout::*;
use crate::commands::status::*;

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
    Escapade,
    // Post
    Heartbeat,
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
        None => println!("No argument provided."),
        _ => panic!()
    }

    Ok(())
}
