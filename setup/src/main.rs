use std::{
    env,
    fs::{self, OpenOptions},
    path::Path,
};


use sqlx::{migrate::Migrator, sqlite::SqlitePoolOptions, Pool, Result, Sqlite};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_name = env::args().nth(1).unwrap_or_else(|| "app.db".to_string());
    let db_path = Path::new(&db_name);

    OpenOptions::new().create(true).write(true).open(db_path)?;

    fs::write(".env", format!("DATABASE_URL=sqlite://{db_name}\nTCP_LOCAL=127.0.0.1:3000\n"))?;

    println!("✓ Database: {db_name}");
    println!("✓ .env created");

    // Migrations
    db_connect(&db_name).await?;

    Ok(())
}


async fn db_connect(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    println!("Connecting to db at {}.", path);
    let db = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(path).await?;

    print!("Checking for migrations... ");
    MIGRATOR.run(&db).await?;

    println!("Done.");
    Ok(db)
}

// Embeds migrations into the compiled binary
// Empty -> default to "./migrations"
static MIGRATOR: Migrator = sqlx::migrate!("../migrations");
