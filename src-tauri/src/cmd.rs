use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::database::establish_connection;
use crate::models;
use crate::workout;

use chrono::NaiveDateTime;

/// Represents a list of workouts.
#[derive(Serialize, Deserialize)]
pub struct WorkoutList {
    /// The list of workouts.
    workouts: Vec<models::Workout>,
}

/// Get all workouts from the database.
///
/// A tauri command that retrieves all workouts from the database.
/// The workouts are returned as a list of `Workout` structs.
///
/// # Example
///
/// ```
/// use workout_tracker::cmd::get_workouts;
///
/// let workouts = get_workouts();
/// ```
///
/// # Panics
///
/// Panics if the workouts cannot be retrieved.
///
/// # Pre-condition
///
/// The connection to the database must be established.
///
/// # Post-condition
///
/// The workouts are returned.
#[tauri::command]
pub fn get_workouts() -> Result<WorkoutList, String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let workouts = match workout::workouts::get_workouts(&mut connection) {
        Ok(workouts) => workouts,
        Err(err) => {
            log::error!("Error getting workouts: {err:?}");
            return Err("Error getting workouts".to_string());
        }
    };
    Ok(WorkoutList { workouts })
}

/// Add a new workout to the database.
///
/// A tauri command that adds a new workout to the database.
/// Creates a new `Workout` struct, and inserts it into the database.
///
/// # Arguments
///
/// * `title` - The title of the workout.
/// * `date` - The date and time when the workout was performed.
///
/// # Example
///
/// ```
/// use workout_tracker::cmd::add_workout;
///
/// add_workout("My Workout", "2021-01-01T12:00");
/// ```
///
/// # Panics
///
/// Panics if the date cannot be parsed.
/// Panics if the workout cannot be created.
///
/// # Pre-condition
///
/// The date must be a valid date, and in the format `YYYY-MM-DD HH:MM:SS`.
/// The connection to the database must be established.
///
/// # Post-condition
///
/// A new workout will be added to the database.
///
#[tauri::command]
pub fn add_workout(title: String, date: &str) -> Result<(), String> {
    let workout_uuid = Uuid::new_v4().hyphenated().to_string();

    let date = match NaiveDateTime::parse_from_str(date, "%Y-%m-%dT%H:%M") {
        Ok(date) => date,
        Err(err) => {
            log::error!("Error parsing date: {err:?}");
            return Err("Error parsing date".to_string());
        }
    };

    let new_workout = models::NewWorkout {
        uuid: workout_uuid,
        title,
        work_date: date,
    };

    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    match workout::workouts::create_workout(&mut connection, &new_workout) {
        Ok(_) => log::info!("Workout Created: {new_workout:?}"),
        Err(err) => {
            log::error!("Error creating workout: {err:?}");
            return Err("Error creating workout".to_string());
        }
    };
    Ok(())
}

/// Get a workout by its uuid from the database.
///
/// A tauri command that retrieves a workout from the database.
/// The workout is returned as a `WorkoutUnique` struct.
///
/// # Arguments
///
/// * `uuid` - The uuid of the workout.
///
/// # Pre-condition
///
/// The connection to the database must be established.
/// The workout must exist in the database.
///
/// # Post-condition
///
/// The workout is returned if it exists in the database or an error is returned if it does not exist.
#[tauri::command]
pub fn get_workout(id: i32) -> Result<models::WorkoutUnique, String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let workout = match workout::workouts::get_workout(&mut connection, id) {
        Ok(workout) => workout,
        Err(err) => {
            log::error!("Error getting workout: {err:?}");
            return Err("Error getting workout".to_string());
        }
    };
    Ok(workout)
}

#[tauri::command]
pub fn delete_workout(id: i32) -> Result<(), String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    match workout::workouts::delete_workout(&mut connection, id) {
        Ok(_) => log::info!("Workout Deleted: {id:?}"),
        Err(err) => {
            log::error!("Error getting workout: {err:?}");
            return Err("Error getting workout".to_string());
        }
    };
    Ok(())
}

#[tauri::command]
pub fn get_exercices(id: i32) -> Result<Vec<models::Exo>, String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let exercices = match workout::exercises::get_exercices(&mut connection, id) {
        Ok(exercices) => exercices,
        Err(err) => {
            log::error!("Error getting exercices: {err:?}");
            return Err("Error getting exercices".to_string());
        }
    };
    Ok(exercices)
}

#[tauri::command]
pub fn add_exercice(
    reps_rep: f64,
    reps_exo: f64,
    poids: Option<f64>,
    exopredef_id: i32,
    workout_id: i32,
) -> Result<(), String> {
    let exo_uuid = Uuid::new_v4().hyphenated().to_string();

    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let new_exercice = models::NewExo {
        uuid: exo_uuid,
        reps_rep,
        reps_exo,
        poids,
        exopredef_id,
        workout_id,
    };

    match workout::exercises::create_exercice(&mut connection, &new_exercice) {
        Ok(_) => log::info!("Exercice Created: {new_exercice:?}"),
        Err(err) => {
            log::error!("Error creating exercice: {err:?}");
            return Err("Error creating exercice".to_string());
        }
    };
    Ok(())
}

/// Represents a list of workouts.
#[derive(Serialize, Deserialize)]
pub struct PredefexoList {
    /// The list of workouts.
    predefexos: Vec<models::ExoPredef>,
}

#[tauri::command]
pub fn get_predefined_exercices() -> Result<PredefexoList, String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let predefexos = match workout::exercises::get_predefined_exercices(&mut connection) {
        Ok(exercices) => {
            log::info!("Exercices: {exercices:?}");
            exercices
        }
        Err(err) => {
            log::error!("Error getting exercices: {err:?}");
            return Err("Error getting exercices".to_string());
        }
    };
    Ok(PredefexoList { predefexos })
}

#[tauri::command]
pub fn get_predefined_exercice(id: i32) -> Result<models::ExoPredef, String> {
    let mut connection = match establish_connection() {
        Ok(connection) => connection,
        Err(err) => {
            log::error!("Error establishing connection: {err:?}");
            return Err("Error establishing connection".to_string());
        }
    };

    let predefexo = match workout::exercises::get_predefined_exercice(&mut connection, id) {
        Ok(exercices) => {
            log::info!("Exercices: {exercices:?}");
            exercices
        }
        Err(err) => {
            log::error!("Error getting exercices: {err:?}");
            return Err("Error getting exercices".to_string());
        }
    };
    Ok(predefexo)
}
