use uuid::Uuid;

use crate::models;
use crate::workout;

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