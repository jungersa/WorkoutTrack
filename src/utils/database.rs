use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use crate::prelude::*;

/// Create the database if it doesn't exist
///
/// # Arguments
///
/// * `db_url` - The database url
///
/// # Returns
///
/// * `Result<()>` - The result of the operation
///
/// # Errors
///
/// * `DatabaseCreationError` - If the database creation fails
async fn create_database(db_url: &str) -> Result<()> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        Sqlite::create_database(db_url)
            .await
            .or_else(|error|  Err(Error::DatabaseCreationError { url: db_url.to_string(), source: error }))?;
    }
    Ok(())
}


/// Connect to the database
///
/// # Arguments
///
/// * `db_url` - The database url
///
/// # Returns
///
/// * `Result<SqlitePool>` - The database pool
///
/// # Errors
///
/// * `DatabaseConnectionError` - If the database connection fails
async fn connect_to_database(db_url: &str) -> Result<SqlitePool> {
    let pool = SqlitePool::connect(db_url)
        .await
        .or_else(|error| return Err(Error::DatabaseConnectionError { url: db_url.to_string(), source: error }))?;
    Ok(pool)
}

/// Migrate the database
///
/// # Arguments
///
/// * `db` - The database pool
///
/// # Returns
///
/// * `Result<()>` - The result of the operation
///
/// # Errors
///
/// * `DatabaseMigrationError` - If the database migration fails
async fn migrate_database(db: &SqlitePool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(db)
        .await
        .or_else(|err| return Err(Error::DatabaseMigrationError {source: err}))?;
    Ok(())
}

/// Create the database if it doesn't exist, connect to it and migrate it
///
/// # Arguments
///
/// * `db_url` - The database url
///
/// # Returns
///
/// * `Result<SqlitePool>` - The database pool
///
/// # Errors
///
/// * `DatabaseCreationError` - If the database creation fails
/// * `DatabaseConnectionError` - If the database connection fails
/// * `DatabaseMigrationError` - If the database migration fails
pub async fn create_and_migrate_database(db_url: &str) -> Result<SqlitePool> {
    println!("Creating database... ");
    create_database(db_url).await?;
    println!("Database created!");

    println!("Connecting to database... ");
    let db = connect_to_database(db_url).await?;
    println!("Database connected!");

    println!("Migrating database... ");
    migrate_database(&db).await?;
    println!("Migration successful!");

    Ok(db)
}