use std::{env, fs};

// Create a .env file for initialization of main
fn main() -> anyhow::Result<()> {
    let db_name = env::args().nth(1).unwrap_or_else(|| "app.db".to_string());
    if fs::exists(format!("./data/{db_name}"))? {
        println!(".env already configured");
    } else {
        fs::write(
            ".env",
            format!("DATABASE_PATH=./data/{db_name}\nDATABASE_URL=sqlite://./data/{db_name}\nBIND_ADDRESS=127.0.0.1:3000\nPUBLIC_URL=http://localhost:3000\nALLOWED_ORIGIN=http://localhost:3000\n"),
        )?;
        println!("✓ .env created");
    }

    Ok(())
}
