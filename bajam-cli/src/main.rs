use clap::{Parser, Subcommand};

pub mod commands;
use commands::{Connection, Credentials};

use crate::commands::{logout, register::{login, register_and_login}, server::set_server, status};

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
    Post,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Input::Server(connection)) => set_server(connection).await?,
        Some(Input::Register(credentials)) => register_and_login(&credentials).await?,
        Some(Input::Login(credentials)) => login(&credentials).await?,
        Some(Input::Logout) => logout().await?,
        Some(Input::Status) => status().await?,
        None => println!("No argument provided."),
        _ => panic!()
    }

    Ok(())
}
