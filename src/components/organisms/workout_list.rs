use yew::prelude::*;

use crate::components::molecules::workout::{Workout, WorkoutProps};

#[derive(Properties, PartialEq, Clone)]
pub struct WorkoutListProps {
    pub workouts: Vec<WorkoutProps>,
}


#[function_component(WorkoutList)]
pub fn workout_list(workout: &WorkoutListProps) -> Html {
    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {for workout.workouts.iter().map(|workout| html! {
                <Workout id={workout.id} uuid={workout.uuid.clone()} title={workout.title.clone()} work_date={workout.work_date} />
            })}
        </div>
    }
}
