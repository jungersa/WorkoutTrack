use super::schema::{workouts, exos};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Represents a workout entity, containing information about a specific workout session.
///
/// This struct is used to store details related to a workout,
/// - **`ID`**: the ID of the workout, uniquely identifying it within a system or database
/// - universally unique identifier (**`UUID`**): a unique identifier for the workout, regardless of its ID within the system
/// - **`title`**: the title or name of the workout
/// - **`work_date`**: the date and time when it was performed
#[derive(Identifiable, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[diesel(table_name = workouts)]
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


/// Represents a workout entity, containing information about a specific workout session and the corresponding exercices.
///
/// This struct is used to store details related to a workout,
/// - **`ID`**: the ID of the workout, uniquely identifying it within a system or database
/// - universally unique identifier (**`UUID`**): a unique identifier for the workout, regardless of its ID within the system
/// - **`title`**: the title or name of the workout
/// - **`work_date`**: the date and time when it was performed
/// - **`exercises`**: the exercises performed during the workout
#[derive(Serialize, Deserialize, Debug)]
pub struct WorkoutUnique {
    /// The ID of the workout, uniquely identifying it within a system or database.
    pub id: i32,

    /// The universally unique identifier (UUID) assigned to the workout.
    pub uuid: String,

    /// The title or name of the workout.
    pub title: String,

    /// The date and time when the workout was performed.
    pub work_date: NaiveDateTime,

    // The exercises performed during the workout.
    pub exercises: Vec<Exo>,
}



/// Represents a exercise entity, containing information about a specific exercise performed during a workout.
///
/// This struct is used to store details related to a exercise,
/// - **`ID`**: the ID of the exercise, uniquely identifying it within a system or database
/// - universally unique identifier (**`UUID`**): a unique identifier for the exercise, regardless of its ID within the system
/// - **`reps_rep`**: the number of sets performed
/// - **`reps_exo`**: the number of reps performed per set
/// - **`poids`**: the weight used$
/// - **`exopredef_id`**: the ID of the predefined exercise
#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Workout))]
#[diesel(table_name = exos)]
pub struct Exo {
    pub id: i32,
    pub uuid: String,
    pub reps_rep: f64, //sets
    pub reps_exo: f64, //reps
    pub poids: Option<f64>,
    pub exopredef_id: i32,
    pub workout_id: i32,
}