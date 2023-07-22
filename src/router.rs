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
    #[at("/workout/:uuid")]
    WorkoutDetail { uuid: String },
    #[at("/workout/create_exo/:uuid")]
    CreateExo { uuid: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::CreateWorkout => html! { <CreateWorkout /> },
        Route::WorkoutDetail { uuid } => {
            html! {<WorkoutDetail { uuid }/>}
        }
        Route::CreateExo { uuid } => {
            html! {<CreateExo { uuid }/>}
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
