use sqlx::{SqlitePool};
use WorkoutTrack::prelude::*;
use WorkoutTrack::utils::database::create_database;

const DB_URL: &str = "sqlite://sqlite.db";


#[tokio::main]
async fn main() -> Result<()> {
    // Create the database if it doesn't exist
    create_database(DB_URL).await?;
    println!("Database created!");

    // Connect to the database
    let db = SqlitePool::connect(DB_URL).await.unwrap();
    println!("Database connected!");

    // Migrate the database
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .or_else(|err| return Err(Error::DatabaseMigrationError {source: err}))?;

    println!("Migration successful!");

    Ok(())
}
