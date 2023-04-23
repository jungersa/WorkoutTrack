use super::schema::messages;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: i32,
    pub uuid: String,
    pub content: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub uuid: String,
    pub content: String,
}
