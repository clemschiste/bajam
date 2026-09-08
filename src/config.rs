use anyhow::Context;
use clap::Parser;
use std::env;

pub fn get_config_from_args() -> anyhow::Result<Config> {
    let args = Args::parse();
    let config = Config::from_args(&args)?;

    Ok(config)
}

#[derive(Parser, Debug)]
pub struct Args {
    /// Use TCP_TAILNET instead of TCP_LOCAL (if existing)
    #[arg(long)]
    pub tailnet: bool,
}

pub struct Config {
    pub tcp_addr: String,
}

impl Config {
    pub fn from_args(args: &Args) -> anyhow::Result<Self> {
        if args.tailnet {
            Ok(
                Self {
                    tcp_addr: env::var("TCP_TAILNET").context("TCP_TAILNET is not configured in the environment file yet. You must define it.")?
                }
            )
        } else {
            Ok(
                Self {
                    tcp_addr: env::var("TCP_LOCAL").context("TCP_LOCAL is not configured anymore in the environment file. You must define it")?
                }
            )
        }
    }
}
