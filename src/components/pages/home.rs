use crate::components::organisms::workout_list::{WorkoutList, WorkoutListProps};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[function_component(Home)]
pub fn home() -> Html {
    let workouts = use_state_eq(|| WorkoutListProps { workouts: vec![] });

    let workouts_clone = workouts.clone();
    spawn_local(async move {
        let workouts_data = invoke("get_workouts", JsValue::NULL)
            .await
            .expect("failed to get workouts");
        let workouts_data: WorkoutListProps = from_value(workouts_data).unwrap();
        workouts_clone.set(workouts_data);
    });

    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <h1 class="text-2xl font-bold mb-4">{"My Workouts"}</h1>
                <WorkoutList workouts={workouts.workouts.clone()} />
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded fixed bottom-0 right-0 mb-8 mr-8">{"Add Workout"}</button>
            </div>
        </body>
    }
}
