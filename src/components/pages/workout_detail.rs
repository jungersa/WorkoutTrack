use crate::components::molecules::bottom_nav::BottomNav;
use crate::types::WorkoutUnique;
use crate::{components::organisms::exo_list::ExoList, router::Route};
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

                <ExoList exos={workout.exercises.clone()} />

                <div>
                    <div class="dropdown dropdown-top dropdown-end fixed bottom-0 right-0 mb-20 mr-8">
                        <div>
                        <label tabindex="0" class="btn m-1 ">
                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="inline-block w-5 h-5 stroke-current"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z"></path></svg>
                        </label>
                        <ul tabindex="0" class="dropdown-content z-[1] menu p-2 shadow bg-slate-100 rounded-box w-52">
                            <li><a onclick={ondelete}>{"Delete"}</a></li>
                            <li><Link<Route> to={Route::WorkoutDetail { uuid: workout.uuid.to_string() }}>{"Edit"}</Link<Route>></li>
                            <li><Link<Route> to={Route::CreateExo { uuid: workout.uuid.to_string() }}>{"Add a Workout"}</Link<Route>></li>
                        </ul>
                        </div>
                    </div>
                 </div>

            </div>
            <BottomNav on_home_page=false on_exo_page=false />
        </body>
    }
}
