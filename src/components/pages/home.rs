use chrono::NaiveDateTime;
use yew::prelude::*;
use crate::components::molecules::workout::{WorkoutProps};
use crate::components::organisms::workout_list::{WorkoutList};


#[function_component(Home)]
pub fn home() -> Html {
    let workout_props = WorkoutProps {
            id: 1,
            uuid: "uuid".to_string(),
            title: "Title".to_string(),
            work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    let workout_props2 = WorkoutProps {
            id: 2,
            uuid: "uuid2".to_string(),
            title: "Title2".to_string(),
            work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    let workout_props3 = WorkoutProps {
            id: 3,
            uuid: "uuid3".to_string(),
            title: "Title3".to_string(),
            work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    let workout_props4 = WorkoutProps {
        id: 4,
        uuid: "uuid4".to_string(),
        title: "Title4".to_string(),
        work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    let workout_props5 = WorkoutProps {
        id: 5,
        uuid: "uuid5".to_string(),
        title: "Title5".to_string(),
        work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    let workout_props6 = WorkoutProps {
        id: 6,
        uuid: "uuid6".to_string(),
        title: "Title6".to_string(),
        work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
    };
    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <h1 class="text-2xl font-bold mb-4">{"My Workouts"}</h1>
                <WorkoutList workouts={vec![workout_props, workout_props2, workout_props3, workout_props4, workout_props5, workout_props6]} />
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded fixed bottom-0 right-0 mb-8 mr-8">{"Add Workout"}</button>
            </div>
        </body>
    }
}