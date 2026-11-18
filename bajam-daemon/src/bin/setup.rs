use std::{env, fs};

// Create a .env file for initialization of main
fn main() -> anyhow::Result<()> {
    let db_name = env::args().nth(1).unwrap_or_else(|| "app.db".to_string());

    fs::write(
        ".env",
        format!("DATABASE_PATH=./data/{db_name}\nDATABASE_URL=./data/sqlite://{db_name}\nTCP_LOCAL=127.0.0.1:3000\n"),
    )?;
    println!("✓ .env created");

    Ok(())
}
