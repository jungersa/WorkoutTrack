use crate::components::molecules::add_workout_form::AddWorkoutForm;

use crate::components::molecules::bottom_nav::BottomNav;
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
    let navigator = use_navigator().expect("No navigator");

    let form_onsubmit = {
        Callback::from(move |workout: WorkoutCreation| {
            spawn_local(async move {
                let args = to_value(&workout.clone()).expect("Couldn't transform the WorkoutData");
                invoke("add_workout", args)
                    .await
                    .expect("failed to add workout");
            });
            navigator.push(&Route::Home);
        })
    };

    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <AddWorkoutForm onsubmit={form_onsubmit}/>
            </div>
            <BottomNav on_home_page=false on_exo_page=false />
        </body>
    }
}
