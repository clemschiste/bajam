use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Input>,
}

#[derive(Subcommand)]
enum Input {
    /// Signup
    Signup(Credentials),
    /// Login
    Login(Credentials),
    // Logout
    Logout,
}

#[derive(Args)]
struct Credentials {
    #[arg(short, long)]
    username: String,
    #[arg(short, long)]
    password: String,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Input::Signup(credentials)) => println!("Welcome, {} !", credentials.username),
        Some(Input::Login(credentials)) => {
            println!(
                "Logged in, always nice to see you {}.",
                credentials.username
            )
        }
        Some(Input::Logout) => println!("Logout."),
        None => println!("No argument provided."),
    }
}
