use clap::{Parser, Subcommand};

pub mod commands;
use commands::{Connection, Credentials};

use crate::commands::{register::register_and_login, server::set_server};

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
    // Server
    Connect(Connection),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Some(Input::Server(connection)) => set_server(connection).await?,
        Some(Input::Register(credentials)) => register_and_login(&credentials).await?,
        Some(Input::Login(credentials)) => {
            println!(
                "Logged in, always nice to see you {}.",
                credentials.username
            )
        }
        Some(Input::Logout) => println!("Logout."),
        Some(Input::Connect(_connection)) => println!("Connect."),
        None => println!("No argument provided."),
    }

    Ok(())
}
