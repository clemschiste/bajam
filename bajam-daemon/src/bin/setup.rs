use bajam_daemon::config::{AppConfig, DatabaseConfig, Environment, EnvironmentConfig};
use std::fs;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

// Create a .env file for initialization of main
fn main() -> anyhow::Result<()> {
    if fs::exists("config.toml")? {
        println!("'config.toml' already configured");
    } else {
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let addr = SocketAddrV4::new(ip, 3000);
        let socket_addr = SocketAddr::V4(addr);
        let config = AppConfig {
            database: DatabaseConfig {
                database_path: "./data/app.db".to_string(),
            },
            environment: EnvironmentConfig {
                environment: Environment::Dev,
            },
            network: bajam_daemon::config::NetworkConfig {
                bind_address: socket_addr,
                public_url: "http://localhost:3000".to_string(),
                allowed_origin: vec![
                    "http://localhost:3000".to_string(),
                    "http://127.0.0.1:3000".to_string(),
                ],
            },
        };

        let content = toml::to_string_pretty(&config)?;
        fs::write("config.toml", content)?;
        println!("'config.toml' created.");
    }

    Ok(())
}
