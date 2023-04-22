use sqlx::{migrate::MigrateDatabase, Error, Sqlite, SqlitePool};
use tauri::AppHandle;
use std::fs;

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
async fn create_database(db_url: &str) -> Result<(), Error> {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        Sqlite::create_database(db_url).await?;
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
async fn connect_to_database(db_url: &str) -> Result<SqlitePool, Error> {
    let pool = SqlitePool::connect(db_url).await?;
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
async fn migrate_database(db: &SqlitePool, handle: &AppHandle) -> Result<(), Error> {
    let resource_path = handle.path_resolver()
      .resolve_resource("migrations/")
      .expect("failed to resolve resource");
    let migrations_path = resource_path.to_str().unwrap();
    let db_init_path = format!("{}/database.sql", migrations_path);
    let db_init = std::fs::read_to_string(db_init_path).expect("Failed to read database init file");
    sqlx::query(&db_init).execute(db).await?;
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
async fn create_and_migrate_database(db_url: &str, handle: &AppHandle) -> Result<SqlitePool, Error> {
    println!("Creating database... {}", db_url);
    create_database(db_url).await?;
    println!("Database created!");

    println!("Connecting to database... ");
    let db = connect_to_database(db_url).await?;
    println!("Database connected!");

    println!("Migrating database... ");
    migrate_database(&db, handle).await?;
    println!("Migration successful!");

    Ok(db)
}


pub async fn initialize_database(handle: &AppHandle) -> Result<SqlitePool, Error> {
    let app_dir = handle.path_resolver().app_data_dir().expect("The app data directory should exist.");
    fs::create_dir_all(&app_dir).expect("The app data directory should be created.");
    let o = app_dir.join("db.sqlite");
    let db_path = o.to_str().unwrap();
    let db_url = format!("sqlite://{}", db_path);

    let db = create_and_migrate_database(&db_url, handle).await?;
    Ok(db)
}

// pub fn add_message(db: &SqlitePool, message: &str) -> Result<(), Error> {
//     sqlx::query!("INSERT INTO Message (content) VALUES (?)", message)
//         .execute(db);
//     Ok(())
// }