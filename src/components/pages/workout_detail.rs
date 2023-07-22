use crate::router::Route;
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
    let navigator = use_navigator().expect("No navigator");
    let workout = use_state_eq(|| WorkoutUnique {
        id: 0,
        uuid: String::new(),
        title: String::new(),
        work_date: NaiveDateTime::default(),
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

    let ondelete = {
        let props = props.clone();
        let navigator: Navigator = navigator.clone();

        Callback::from(move |_: MouseEvent| {
            let args = to_value(&props.clone()).expect("Couldn't transform the WorkoutData");
            spawn_local(async move {
                invoke("delete_workout", args)
                    .await
                    .expect("failed to delete workout");
            });
            navigator.push(&Route::Home);
        })
    };

    html! {
            <body class="item-center">

                <div class="container mx-auto py-8">
                    <h1 class="text-2xl font-bold mb-4">{ &workout.title }</h1>
                    <p class="text-gray-600 mb-2">{ "Date: "} { &workout.work_date.format("%c").to_string() }</p>

                    <div class="bg-slate-100 rounded shadow-md p-4">
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
                    <div>
                        <div class="dropdown dropdown-top dropdown-end fixed bottom-0 right-0 mb-20 mr-8">
                            <div>
                            <label tabindex="0" class="btn m-1 ">
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
                            </label>
                            <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-slate-100 rounded-box w-52">
                                <li><a onclick={ondelete}>{"Delete"}</a></li>
                                <li><Link<Route> to={Route::WorkoutDetail { uuid: workout.uuid.to_string() }}>{"Edit"}</Link<Route>></li>
                                <li><Link<Route> to={Route::WorkoutDetail { uuid: workout.uuid.to_string() }}>{"Add a Workout"}</Link<Route>></li>
                            </ul>
                            </div>
                        </div>
                     </div>

                </div>
                <div class="btm-nav bg-slate-50">
                    <Link<Route> to={Route::Home}>
                    <button>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" /></svg>
                    </button></Link<Route>>
                    <button>
                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" /></svg>
                    </button>
                </div>


            </body>
        }
}
