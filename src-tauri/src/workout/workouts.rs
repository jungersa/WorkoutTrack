use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models;
use crate::schema;

pub struct Workout {}

impl Workout {
    /// Get all workouts
    pub fn get_workouts() -> Vec<models::Workout> {
        use schema::workouts::dsl::*;

        let connection = &mut establish_connection();
        let results = workouts
            .load::<models::Workout>(connection)
            .expect("Error loading workouts");

        results
    }

    /// Get a workout by its uuid
    pub fn get_workout_by_uuid(uuid: &str) -> models::Workout {
        let connection = &mut establish_connection();
        let result = schema::workouts::dsl::workouts
            .filter(schema::workouts::uuid.eq(uuid))
            .first::<models::Workout>(connection)
            .expect("Error loading workout");

        result
    }

    /// Create a new workout
    pub fn create_workout(new_workout: models::NewWorkout) -> () {
        use schema::workouts::dsl::*;

        let connection = &mut establish_connection();
        diesel::insert_into(workouts)
            .values(&new_workout)
            .execute(connection)
            .expect("Error saving new workout");
    }

    /// Delete a workout
    pub fn delete_workout(uuid: &str) -> () {
        let connection = &mut establish_connection();
        diesel::delete(schema::workouts::dsl::workouts.filter(schema::workouts::uuid.eq(uuid)))
            .execute(connection)
            .expect("Error deleting workout");
    }
}
