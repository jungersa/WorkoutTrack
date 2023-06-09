use workout_track::prelude::*;
use workout_track::utils::database::create_and_migrate_database;

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() -> Result<()> {
    let _pool = create_and_migrate_database(DB_URL).await?;

    Ok(())
}
