use sqlx::{SqlitePool};
use sqlx::{Pool, Result, Sqlite, sqlite::SqlitePoolOptions, migrate::Migrator};
use bajam_common::models::heartbeat::Heartbeat;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub history: Vec<Heartbeat>,
}

// A voir si je souhaite tracer la ligne. Probablement pas maintenant mais il faudrait retourner dans le bon ordre
impl AppState {
    pub async fn new(db: SqlitePool) -> Result<Self, sqlx::Error> {
        // Le problème quand tu relance le server, il intègre la "current pos" dans l'history
        print!("Loading history... ");

        // OFFSET 1 car on ne veut pas intégrer le plus récent current à l'history
        let history = sqlx::query_as!(
            Heartbeat,
            r#"
                SELECT latitude, longitude, timestamp, description, picture_id, user_id, escapade_id
                FROM heartbeats
                ORDER BY timestamp DESC
                LIMIT 100 OFFSET 1
            "#
        )
        .fetch_all(&db)
        .await?;

        println!("Done.");

        Ok(Self {
            db,
            history,
        })

    }
}

use std::{fs::OpenOptions, path::Path};

pub async fn db_connect(path: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    // Check if path exists
    let db_exists = Path::new(path).try_exists()?;

    // If it does not exists, create it
    if !db_exists {
        print!("Creating a new database in {}... ", path);

        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(path)
            .map_err(sqlx::Error::Io)?;
    } else {
        println!("Using existing database in {}.", path);
    }

    let database_url = format!("sqlite://{}", path);

    print!("Connecting to db at {}... ", database_url);

    // this piece of code is also common to bin/setup.rs file
    let db = SqlitePoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await?;

    print!("Checking for migrations... ");
    MIGRATOR.run(&db).await?;

    println!("Done.");
    Ok(db)

}
// Embeds migrations into the compiled binary
// Empty -> default to "./migrations"
static MIGRATOR: Migrator = sqlx::migrate!();
