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

#[tauri::command]
pub async fn get_workouts() -> Vec<models::Workout> {
    workout::workouts::Workout::get_workouts()
}

#[tauri::command]
pub async fn add_workout(title: String, work_date: String) -> () {
    let workout_uuid = Uuid::new_v4().hyphenated().to_string();
    let work_date = NaiveDateTime::parse_from_str(&work_date, "%Y-%m-%d %H:%M:%S").unwrap();

    let new_workout = models::NewWorkout {
        uuid: workout_uuid,
        title: title,
        work_date: work_date,
    };

    workout::workouts::Workout::create_workout(new_workout)
}