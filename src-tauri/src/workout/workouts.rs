use diesel::prelude::*;

use crate::database::establish_connection;
use crate::errors::{DatabaseError, Error};
use crate::models;
use crate::schema;

pub struct Workout {}

impl Workout {
    /// Get all the workouts in the database
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
}
