use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::models;
use crate::workout;

use chrono::NaiveDateTime;

#[tauri::command]
pub async fn get_messages() -> Vec<models::Message> {
    workout::messages::Message::get_messages()
}

#[tauri::command]
pub async fn add_message(message: String) -> () {
    let message_uuid = Uuid::new_v4().hyphenated().to_string();

    let new_message = models::NewMessage {
        uuid: message_uuid,
        content: message,
    };

    workout::messages::Message::create_message(new_message)
}

#[derive(Serialize, Deserialize)]
pub struct WorkoutList {
    workouts: Vec<models::Workout>,
}

#[tauri::command]
pub async fn get_workouts() -> WorkoutList {
    let workouts = workout::workouts::Workout::get_workouts();
    WorkoutList { workouts }
}

#[tauri::command]
pub async fn add_workout(title: String, date: String) -> () {
    let workout_uuid = Uuid::new_v4().hyphenated().to_string();
    let date = NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S").unwrap();

    let new_workout = models::NewWorkout {
        uuid: workout_uuid,
        title: title,
        work_date: date,
    };

    workout::workouts::Workout::create_workout(new_workout)
}
