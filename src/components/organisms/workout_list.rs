use yew::prelude::*;

use crate::components::molecules::workout::{Workout, WorkoutProps};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct WorkoutListProps {
    pub workouts: Vec<WorkoutProps>,
}

#[function_component(WorkoutList)]
pub fn workout_list(props: &WorkoutListProps) -> Html {
    let WorkoutListProps { workouts } = props.clone();

    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {for workouts.iter().map(|workout| html! {
                <Workout ..workout.clone() />
            })}
        </div>
    }
}
