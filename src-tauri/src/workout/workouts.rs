use diesel::prelude::*;

use crate::database::establish_connection;
use crate::models;
use crate::schema;

pub struct Workout {}

impl Workout {
    /// Get all workouts
    pub fn get_workouts() -> Vec<models::Workout> {
        use crate::schema::workouts::dsl::workouts;

        let connection = &mut establish_connection();
        workouts
            .load::<models::Workout>(connection)
            .expect("Error loading workouts")
    }

    /// Get a workout by its uuid
    pub fn get_workout_by_uuid(uuid: &str) -> models::Workout {
        let connection = &mut establish_connection();

        schema::workouts::dsl::workouts
            .filter(schema::workouts::uuid.eq(uuid))
            .first::<models::Workout>(connection)
            .expect("Error loading workout")
    }

    /// Create a new workout
    pub fn create_workout(new_workout: &models::NewWorkout) {
        use crate::schema::workouts::dsl::workouts;

        let connection = &mut establish_connection();
        diesel::insert_into(workouts)
            .values(new_workout)
            .execute(connection)
            .expect("Error saving new workout");
    }

    /// Delete a workout
    pub fn delete_workout(uuid: &str) {
        let connection = &mut establish_connection();
        diesel::delete(schema::workouts::dsl::workouts.filter(schema::workouts::uuid.eq(uuid)))
            .execute(connection)
            .expect("Error deleting workout");
    }
}
