use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::pages::{
    create_exo::CreateExo, create_workout::CreateWorkout, home::Home, workout_detail::WorkoutDetail,
};

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/create_workout")]
    CreateWorkout,
    #[at("/workout/:workout_id")]
    WorkoutDetail { workout_id: i32 },
    #[at("/workout/create_exo/:workout_id")]
    CreateExo { workout_id: i32 },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateWorkout => html! { <CreateWorkout /> },
        Route::WorkoutDetail { workout_id } => {
            html! {<WorkoutDetail { workout_id }/>}
        }
        Route::CreateExo { workout_id } => {
            html! {<CreateExo { workout_id }/>}
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
