use sqlx::{migrate::MigrateDatabase, Sqlite};
use crate::prelude::*;

pub async fn create_database(db_url: &str) -> Result<()> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => return Ok(()),
            Err(error) => {
                return Err(Error::DatabaseCreationError { url: db_url.to_string(), source: error });
            }
        }
    }
    Ok(())
}