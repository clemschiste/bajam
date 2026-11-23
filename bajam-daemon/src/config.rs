use anyhow::Context;
use clap::Parser;
use std::env;

pub fn get_config() -> anyhow::Result<AppConfig> {
    // let _args = Args::parse();
    let config = AppConfig::from_env()?;

    Ok(config)
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Mode {
    Dev,
    Prod,
}

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub bind_address: String,
    pub allowed_origin: String, // maybe a vec later with a toml config instead of env
    pub public_url: String,
    pub database_path: String,
    pub database_url: String,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            bind_address: env::var("BIND_ADDRESS")
                .context("BIND_ADDRESS is not configured in .env file.")?,
            allowed_origin: env::var("ALLOWED_ORIGIN")
                .context("ALLOWED_ORIGIN not defined in .env file.")?,
            public_url: env::var("PUBLIC_URL")
                .context("PUBLIC_ADRESS not defined in .env file.")?,
            database_path: env::var("DATABASE_PATH")
                .context("DATABASE_PATH not defined in .env file.")?,
            database_url: env::var("DATABASE_URL")
                .context("DATABASE_URL not defined in .env file.")?,
        })
    }

    #[allow(unused)]
    /// For testing use
    pub fn empty() -> AppConfig {
        AppConfig {
            bind_address: "".to_string(),
            allowed_origin: "".to_string(),
            public_url: "".to_string(),
            database_path: "".to_string(),
            database_url: "".to_string(),
        }
    }
}
