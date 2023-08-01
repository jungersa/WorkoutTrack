use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use yew::{html, AttrValue, Properties};

#[derive(Properties, PartialEq, Clone, Debug, Eq)]
pub struct WorkoutProps {
    pub id: i32,
    pub uuid: AttrValue,
    pub title: AttrValue,
    pub work_date: NaiveDateTime,
}

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct WorkoutCreation {
    pub title: String,
    pub date: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct WorkoutVec {
    pub workouts: Vec<Workout>,
}

impl html::IntoPropValue<Vec<WorkoutProps>> for WorkoutVec {
    fn into_prop_value(self) -> Vec<WorkoutProps> {
        self.workouts
            .into_iter()
            .map(|workout| WorkoutProps {
                id: workout.id,
                uuid: AttrValue::from(workout.uuid),
                title: AttrValue::from(workout.title),
                work_date: workout.work_date,
            })
            .collect()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Workout {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub work_date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
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

#[derive(Properties, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Exo {
    pub id: i32,
    pub uuid: String,
    pub reps_rep: f64, //sets
    pub reps_exo: f64, //reps
    pub poids: Option<f64>,
    pub exopredef_id: i32,
    pub workout_id: i32,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct PredefExo {
    pub id: i32,
    pub uuid: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct PredefExoVec {
    pub predefexos: Vec<PredefExo>,
}
