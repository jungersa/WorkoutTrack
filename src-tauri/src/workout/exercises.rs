use crate::schema;
use crate::{
    errors::{DatabaseError, Error},
    models,
};
use diesel::prelude::*;

/// Get all the exercice related to a workout in the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
/// * `workout_id` - The id of the workout
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
/// The exercices related to the workout are returned as a list of `Exo` structs.
///
pub fn get_exercices(
    connection: &mut diesel::SqliteConnection,
    workout_id: i32,
) -> Result<Vec<models::Exo>, Error> {
    use crate::schema::exos::dsl::exos;

    exos.filter(schema::exos::workout_id.eq(workout_id))
        .load::<models::Exo>(connection)
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}

/// Create an exercice in the database
///
/// # Arguments
///
/// * `connection` - A mutable reference to a `diesel::SqliteConnection` struct that holds the connection to the database
/// * `exo` - The exercice to create
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
/// The exercice is created in the database.
pub fn create_exercice(
    connection: &mut diesel::SqliteConnection,
    exo: &models::NewExo,
) -> Result<(), Error> {
    use crate::schema::exos::dsl::exos;

    diesel::insert_into(exos)
        .values(exo)
        .execute(connection)
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))?;

    Ok(())
}

/// Get all the exercice predefinitions from the database
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
/// The exercice predefinitions are returned as a list of `ExoPredef` structs.
///
pub fn get_predefined_exercices(
    connection: &mut diesel::SqliteConnection,
) -> Result<Vec<models::ExoPredef>, Error> {
    use crate::schema::exopredefs::dsl::exopredefs;

    exopredefs
        .load::<models::ExoPredef>(connection)
        .map_err(|err| Error::DatabaseError(DatabaseError::QueryError(err)))
}
