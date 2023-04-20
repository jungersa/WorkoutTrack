use WorkoutTrack::prelude::*;
use WorkoutTrack::utils::database::create_database;

const DB_URL: &str = "sqlite://sqlite.db";


#[tokio::main]
async fn main() -> Result<()> {
    // Create the database if it doesn't exist
    create_database(DB_URL).await?;

    Ok(())
}
