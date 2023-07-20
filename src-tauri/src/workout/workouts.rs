use diesel::prelude::*;

use crate::errors::{DatabaseError, Error};
use crate::models;
use crate::schema;

/// Get all the workouts in the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
///
/// # Errors
///
/// If the connection to the database cannot be established, `DatabaseError` variant is returned.
/// If the query to the database fails, `DatabaseError::QueryError` variant is returned.
///
/// # Pre-conditions
///
/// The database must be created.
/// The connection to the database must be established.
///
/// # Post-conditions
///
/// The workouts are returned as a list of `Workout` structs.
///
pub fn get_workouts(
    connection: &mut diesel::SqliteConnection,
) -> Result<Vec<models::Workout>, Error> {
    use crate::schema::workouts::dsl::workouts;

    workouts
        .load::<models::Workout>(connection)
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}

/// Get a workout by its uuid from the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
/// * `uuid` - A string slice that holds the uuid of the workout
///
/// # Errors
///
/// If the connection to the database cannot be established, `DatabaseError` variant is returned.
/// If the query to the database fails, `DatabaseError::QueryError` variant is returned.
///
/// # Pre-conditions
///
/// The database must be created.
/// The connection to the database must be established.
/// A workout with the given uuid must exist in the database.
///
/// # Post-conditions
///
/// The workout is returned as a `Workout` struct.
///
pub fn get_workout_by_uuid(
    connection: &mut diesel::SqliteConnection,
    uuid: &str,
) -> Result<models::Workout, Error> {
    schema::workouts::dsl::workouts
        .filter(schema::workouts::uuid.eq(uuid))
        .first::<models::Workout>(connection)
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}

/// Create a new workout in the database
///
/// The new workout is created with the data from the `NewWorkout` struct, and a new ID.
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
/// * `new_workout` - A reference to a `NewWorkout` struct that holds the data to create the new workout in the database
///
/// # Errors
///
/// If the connection to the database cannot be established, `DatabaseError` variant is returned.
/// If the query to the database fails, `DatabaseError::QueryError` variant is returned.
///
/// # Pre-conditions
///
/// The database must be created.
/// A workout with the given uuid must not exist in the database.
/// The connection to the database must be established.
///
/// # Post-conditions
///
/// The new workout is created in the database, with the data from the `NewWorkout` struct, and a new ID.
///
pub fn create_workout(
    connection: &mut diesel::SqliteConnection,
    new_workout: &models::NewWorkout,
) -> Result<(), Error> {
    use crate::schema::workouts::dsl::workouts;

    diesel::insert_into(workouts)
        .values(new_workout)
        .execute(connection)
        .map(|_| ())
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}

/// Delete a workout from the database by its uuid
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
/// * `uuid` - A string slice that holds the uuid of the workout which will be deleted
///
/// # Errors
///
/// If the connection to the database cannot be established, `DatabaseError` variant is returned.
/// If the query to the database fails, `DatabaseError::QueryError` variant is returned.
///
/// # Pre-conditions
///
/// The database must be created.
/// A workout with the given uuid must exist in the database.
/// The connection to the database must be established.
///
pub fn delete_workout(connection: &mut diesel::SqliteConnection, uuid: &str) -> Result<(), Error> {
    diesel::delete(schema::workouts::dsl::workouts.filter(schema::workouts::uuid.eq(uuid)))
        .execute(connection)
        .map(|_| ())
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::workouts;
    use chrono::NaiveDateTime;
    use diesel::{insert_into, prelude::*, result::Error as DieselError, sqlite::SqliteConnection};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

    // Helper function to establish an in-memory SQLite connection for testing
    fn establish_connection() -> SqliteConnection {
        let mut connection = SqliteConnection::establish(":memory:").unwrap();
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Error migrating");
        connection
    }

    #[test]
    fn test_create_workout_success() {
        let connection = &mut establish_connection();

        // Create a NewWorkout instance with sample data
        let new_workout = models::NewWorkout {
            uuid: "uuid".to_string(),
            title: "title".to_string(),
            work_date: NaiveDateTime::default(),
        };

        // Call the create_workout function and expect Ok(())
        assert_eq!(create_workout(connection, &new_workout), Ok(()));

        // Perform assertions to verify that the workout is created in the database as expected
        let workout_count: i64 = workouts::table.count().get_result(connection).unwrap();
        assert_eq!(workout_count, 1);
    }

    #[test]
    fn test_get_workouts_success() {
        let connection = &mut establish_connection();

        // Perform test setup: Insert some sample workouts into the database
        let workout1 = models::NewWorkout {
            uuid: "uuid1".to_string(),
            title: "title1".to_string(),
            work_date: NaiveDateTime::default(),
        };
        diesel::insert_into(workouts::table)
            .values(&workout1)
            .execute(connection)
            .unwrap();

        let workout2 = models::NewWorkout {
            uuid: "uuid2".to_string(),
            title: "title2".to_string(),
            work_date: NaiveDateTime::default(),
        };
        diesel::insert_into(workouts::table)
            .values(&workout2)
            .execute(connection)
            .unwrap();

        // Call the get_workouts function and expect a vector of Workout instances
        let result = get_workouts(connection).unwrap();

        // Perform assertions to verify the correctness of the returned workouts
        assert_eq!(result.len(), 2);
    }

}
