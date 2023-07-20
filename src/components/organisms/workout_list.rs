use yew::prelude::*;

use crate::components::molecules::workout::Workout;
use crate::types::WorkoutProps;

#[derive(Properties, PartialEq, Clone, Debug, Eq)]
pub struct Props {
    pub workouts: Vec<WorkoutProps>,
}

#[function_component(WorkoutList)]
pub fn workout_list(props: &Props) -> Html {
    let Props { workouts } = props.clone();

    html! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
            {for workouts.iter().map(|workout| html! {
                <Workout ..workout.clone() />
            })}
        </div>
    }
}