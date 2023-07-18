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

/// Represents a workout entity, containing information about a specific workout session.
///
/// This struct is used to store details related to a workout,
/// - **`ID`**: the ID of the workout, uniquely identifying it within a system or database
/// - universally unique identifier (**`UUID`**): a unique identifier for the workout, regardless of its ID within the system
/// - **`title`**: the title or name of the workout
/// - **`work_date`**: the date and time when it was performed
#[derive(Queryable, Serialize, Deserialize, Debug)]
pub struct Workout {
    /// The ID of the workout, uniquely identifying it within a system or database.
    pub id: i32,

    /// The universally unique identifier (UUID) assigned to the workout.
    pub uuid: String,

    /// The title or name of the workout.
    pub title: String,

    /// The date and time when the workout was performed.
    pub work_date: NaiveDateTime,
}

/// Represents data to create a new workout entity.
///
/// This struct is used to create a new workout,
/// - universally unique identifier (**`UUID`**): a unique identifier for the workout, regardless of its ID within the system
/// - **`title`**: the title or name of the workout
/// - **`work_date`**: the date and time when it was performed
#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = workouts)]
pub struct NewWorkout {
    /// The universally unique identifier (UUID) assigned to the workout.
    pub uuid: String,

    /// The title or name of the workout.
    pub title: String,

    /// The date and time when the workout was performed.
    pub work_date: NaiveDateTime,
}
