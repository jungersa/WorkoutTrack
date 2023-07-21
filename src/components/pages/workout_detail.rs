use crate::{router::Route, types::_WorkoutProps::work_date};
use crate::types::WorkoutUnique;
use chrono::NaiveDateTime;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Properties, Serialize, PartialEq, Clone, Debug, Eq)]
pub struct Props {
    pub uuid: String,
}

#[function_component(WorkoutDetail)]
pub fn workout_detail(props: &Props) -> Html {
    let workout = use_state_eq(|| WorkoutUnique {
        id: 0,
        uuid: "".to_string(),
        title: "".to_string(),
        work_date: NaiveDateTime::from_timestamp_opt(0, 0).unwrap(),
        exercises: vec![],
    });


    {
        let workout = workout.clone();
        let props = props.clone();
        spawn_local(async move {
            let args = to_value(&props.clone()).expect("Couldn't transform the WorkoutData");
            let workout_data = invoke("get_workout", args)
                .await
                .expect("failed to get workouts");

            let workout_data: WorkoutUnique =
                from_value(workout_data).expect("Couldn't transform the WorkoutUnique");
            workout.set(workout_data);
        });
    };

    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <h1 class="text-2xl font-bold mb-4">{ &workout.title }</h1>
                <p class="text-gray-600 mb-2">{ "Date: "} { &workout.work_date.format("%c").to_string() }</p>

                <div class="bg-white rounded shadow-md p-4">
                    <h2 class="text-lg font-bold mb-2">{"Exercises"}</h2>
                    {for workout.exercises.iter().map(|exercise| html! {
                        <div class="border rounded p-2">
                            <h3 class="text-md font-semibold mb-2">{ exercise.exopredef_id }</h3>
                            <p class="mb-1">{"Sets: "} { exercise.reps_rep }</p>
                            <p class="mb-1">{"Reps: "}{ exercise.reps_exo }</p>
                            <p class="mb-1">{"Weight: "} { exercise.poids }</p>
                        </div>
                    })}
                </div>
                <Link<Route> to={Route::Home}><button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded fixed bottom-0 right-0 mb-8 mr-8">{"Back"}</button></Link<Route>>
            </div>
        </body>
    }
}
