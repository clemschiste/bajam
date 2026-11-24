use clap::Parser;
use crate::commands::heartbeat::*;
use crate::commands::server::set_server;
use crate::commands::register::*;
use crate::commands::login::*;
use crate::commands::logout::*;
use crate::commands::status::*;
use crate::commands::escapades::*;
use crate::args::escapade::*;
use crate::args::heartbeat::*;
use crate::args::{Input, Cli};

mod commands;
mod errors;
mod args;

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
                SubEscapades::Show => (),
                SubEscapades::Delete => (),
            }
        },
        Some(Input::Heartbeat{ command: action }) => {
            match action {
                SubHeartbeat::Create(args) => {
                    if args.valid_latitude()? && args.valid_name()? {
                        create_heartbeat(args.escapade, args.latitude, args.longitude, args.description).await?;
                    }
                }
                SubHeartbeat::List(arg) => { list_heartbeats_from(arg.escapade).await? },
                SubHeartbeat::Show(_) => (),
            }
        },
        None => println!("No argument provided."),
    }

    Ok(())
}
