use super::schema::{messages, workouts};
use chrono::NaiveDateTime;
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

#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Workout {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub work_date: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = workouts)]
pub struct NewWorkout {
    pub uuid: String,
    pub title: String,
    pub work_date: NaiveDateTime,
}
