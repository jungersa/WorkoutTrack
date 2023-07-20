use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::database::establish_connection;
use crate::models;
use crate::workout;

use chrono::NaiveDateTime;

#[tauri::command]
pub fn get_messages() -> Vec<models::Message> {
    workout::messages::Message::get_messages()
}

#[tauri::command]
pub fn add_message(message: String) {
    let message_uuid = Uuid::new_v4().hyphenated().to_string();

    let new_message = models::NewMessage {
        uuid: message_uuid,
        content: message,
    };

    workout::messages::Message::create_message(&new_message);
}

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
pub fn get_workout() -> Result<WorkoutList, String> {
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
    return Ok(());
}