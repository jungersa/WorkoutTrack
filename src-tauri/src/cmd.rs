use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

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

#[derive(Serialize, Deserialize)]
pub struct WorkoutList {
    workouts: Vec<models::Workout>,
}

#[tauri::command]
pub fn get_workouts() -> WorkoutList {
    let workouts = workout::workouts::Workout::get_workouts();
    WorkoutList { workouts }
}

#[tauri::command]
pub fn add_workout(title: String, date: &str) {
    let workout_uuid = Uuid::new_v4().hyphenated().to_string();
    let date =
        NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S").expect("Error parsing date");

    let new_workout = models::NewWorkout {
        uuid: workout_uuid,
        title,
        work_date: date,
    };

    workout::workouts::Workout::create_workout(&new_workout);
}
