use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models;
use crate::schema;

pub struct Message {}

impl Message {
    /// Get all messages
    pub fn get_messages() -> Vec<models::Message> {
        use schema::messages::dsl::*;

        let connection = &mut establish_connection();
        let results = messages
            .load::<models::Message>(connection)
            .expect("Error loading messages");

        results
    }

    /// Get a message by its uuid
    pub fn get_message_by_uuid(uuid: &str) -> models::Message {
        let connection = &mut establish_connection();
        let result = schema::messages::dsl::messages
            .filter(schema::messages::uuid.eq(uuid))
            .first::<models::Message>(connection)
            .expect("Error loading message");

        result
    }

    /// Create a new message
    pub fn create_message(new_message: models::NewMessage) -> () {
        use schema::messages::dsl::*;

        let connection = &mut establish_connection();
        diesel::insert_into(messages)
            .values(&new_message)
            .execute(connection)
            .expect("Error saving new message");
    }

    /// Delete a message
    pub fn delete_message(uuid: &str) -> () {
        let connection = &mut establish_connection();
        diesel::delete(schema::messages::dsl::messages.filter(schema::messages::uuid.eq(uuid)))
            .execute(connection)
            .expect("Error deleting message");
    }
}
