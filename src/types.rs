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
