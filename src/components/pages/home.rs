use crate::components::{molecules::workout::WorkoutProps, organisms::workout_list::WorkoutList};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct WorkoutVec {
    pub workouts: Vec<Workout>,
}

impl html::IntoPropValue<Vec<WorkoutProps>> for WorkoutVec {
    fn into_prop_value(self) -> Vec<WorkoutProps> {
        self.workouts
            .into_iter()
            .map(|workout| WorkoutProps {
                id: workout.id,
                uuid: AttrValue::from(workout.uuid),
                title: AttrValue::from(workout.title),
                work_date: workout.work_date,
            })
            .collect()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Workout {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub work_date: NaiveDateTime,
}

#[function_component(Home)]
pub fn home() -> Html {
    // Change to props!()
    let workouts = use_state_eq(|| WorkoutVec { workouts: vec![] });

    let workouts_clone = workouts.clone();
    spawn_local(async move {
        let workouts_data = invoke("get_workouts", JsValue::NULL)
            .await
            .expect("failed to get workouts");
        let workouts_data: WorkoutVec = from_value(workouts_data).unwrap();
        workouts_clone.set(workouts_data);
    });

    html! {
        <body class="bg-gray-100">
            <div class="container mx-auto py-8">
                <h1 class="text-2xl font-bold mb-4">{"My Workouts"}</h1>
                <WorkoutList workouts={(*workouts).clone()} />
                <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded fixed bottom-0 right-0 mb-8 mr-8">{"Add Workout"}</button>
            </div>
        </body>
    }
}
