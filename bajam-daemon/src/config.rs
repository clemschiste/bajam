use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

pub fn get_config() -> anyhow::Result<AppConfig> {
    let config = AppConfig::from_toml_config("config.toml")?;
    Ok(config)
}

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub environment: Environment,
}

#[derive(Clone, Copy, Debug, clap::ValueEnum, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Prod,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub environment: EnvironmentConfig,
    pub database: DatabaseConfig,
    pub network: NetworkConfig,
}

impl AppConfig {
    pub fn from_toml_config(path: &str) -> anyhow::Result<Self> {
        let config_toml = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&config_toml)?;

        Ok(config)
    }

    #[allow(unused)]
    /// For testing use
    pub fn empty() -> AppConfig {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let addr = SocketAddrV4::new(ip, 3001);
        let socket_addr = SocketAddr::V4(addr);
        AppConfig {
            environment: {
                EnvironmentConfig {
                    environment: Environment::Dev,
                }
            },
            database: DatabaseConfig {
                database_path: "".to_string(),
            },
            network: NetworkConfig {
                bind_address: socket_addr,
                public_url: "".to_string(),
                allowed_origin: vec!["".to_string()],
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub database_path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub environment: Environment,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub bind_address: SocketAddr,
    pub public_url: String,
    pub allowed_origin: Vec<String>,
}
