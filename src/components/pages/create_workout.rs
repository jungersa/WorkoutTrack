use crate::components::molecules::add_workout_form::AddWorkoutForm;

use crate::router::Route;
use crate::types::WorkoutCreation;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[function_component(CreateWorkout)]
pub fn create_workout() -> Html {
    let navigator = use_navigator().unwrap();

    let form_onsubmit = {
        let navigator = navigator.clone();
        Callback::from(move |workout: WorkoutCreation| {
            spawn_local(async move {
                let args = to_value(&workout.clone()).expect("Couldn't transform the WorkoutData");
                invoke("add_workout", args)
                    .await
                    .expect("failed to add workout");
            });
            navigator.push(&Route::Home)
        })
    };

    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <AddWorkoutForm onsubmit={form_onsubmit}/>
                <Link<Route> to={Route::Home}><button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded fixed bottom-0 right-0 mb-8 mr-8">{"Add Workout"}</button></Link<Route>>
            </div>
        </body>
    }
}
