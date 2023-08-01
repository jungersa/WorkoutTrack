use crate::{
    components::molecules::{bottom_nav::BottomNav, exo::Exo},
    router::Route,
    types::{ExoCreation, PredefExo, PredefExoVec},
};
use gloo_console::log;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Properties, Serialize, PartialEq, Clone, Debug, Eq)]
pub struct Props {
    pub workout_id: i32,
}

#[function_component(CreateExo)]
pub fn create_exo(props: &Props) -> Html {
    let navigator = use_navigator().expect("No navigator");
    let predefexos: UseStateHandle<Vec<PredefExo>> = use_state_eq(|| vec![]);
    let state: UseStateHandle<ExoCreation> = use_state(|| ExoCreation {
        workoutId: props.workout_id,
        ..ExoCreation::default()
    });

    {
        let predefexos = predefexos.clone();
        if predefexos.is_empty() {
            spawn_local(async move {
                let predefexos_data = invoke("get_predefined_exercices", JsValue::NULL)
                    .await
                    .expect("failed to get workouts");
                let predefexos_data: PredefExoVec = from_value(predefexos_data)
                    .expect("Couldn't transform the data to PredefExoVec");
                predefexos.set(predefexos_data.predefexos);
                log!("predefexos_data");
            });
        }
    };

    let onsubmit = {
        let state = state.clone();
        let props = props.clone();

        Callback::from(move |event: SubmitEvent| {
            let state = state.clone();
            event.prevent_default();

            spawn_local(async move {
                let args = to_value(&(*state).clone())
                    .expect("Couldn't transform the ExoCreation to JsValue");
                invoke("add_exercice", args)
                    .await
                    .expect("failed to add workout");
            });

            navigator.push(&Route::WorkoutDetail {
                workout_id: props.workout_id,
            });
        })
    };

    let set_changed = {
        let state = state.clone();

        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                state.set(ExoCreation {
                    repsRep: input.value().parse().expect("Couldn't parse the reps_rep"),
                    ..(*state).clone()
                });
            }
        })
    };

    let reps_changed = {
        let state = state.clone();

        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                state.set(ExoCreation {
                    repsExo: input.value().parse().expect("Couldn't parse the reps_rep"),
                    ..(*state).clone()
                });
            }
        })
    };

    let weight_changed = {
        let state = state.clone();

        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                state.set(ExoCreation {
                    poids: Some(input.value().parse().expect("Couldn't parse the reps_rep")),
                    ..(*state).clone()
                });
            }
        })
    };

    let exopredef_changed = {
        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                state.set(ExoCreation {
                    exopredefId: input.value().parse().expect("Couldn't parse the reps_rep"),
                    ..(*state).clone()
                });
            }
        })
    };

    html! {
        <body class="item-center">
            <div class="container mx-auto py-8">
                <button type="" class="btn btn-primary">{"GoBack"}</button>
                <h1 class="text-3xl font-bold mb-2">{"Create a new exercise"}</h1>
                <form class="bg-white p-6 rounded shadow-md" {onsubmit}>
                    <div class="mb-4">
                        <label for="reps_rep" class="block text-gray-700 font-bold mb-2">{"Number of sets:"}</label>
                        <input onchange={set_changed} type="number" min="0" name="reps_rep" id="reps_rep" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true/>
                    </div>

                    <div class="mb-4">
                        <label for="reps_exo" class="block text-gray-700 font-bold mb-2">{"Number of reps:"}</label>
                        <input onchange={reps_changed} type="number" min="0" name="reps_exo" id="reps_exo" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true/>
                    </div>

                    <div class="mb-4">
                        <label for="poids" class="block text-gray-700 font-bold mb-2">{"Poids:"}</label>
                        <input onchange={weight_changed} type="number" min="0" name="poids" id="poids" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300"/>
                    </div>

                    <div class="mb-4">
                        <label for="exopredef_id" class="block text-gray-700 font-bold mb-2">{"Exercise:"}</label>
                        <select onchange={exopredef_changed} name="exopredef_id" id="exopredef_id" class="form-select w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true>
                            {
                                predefexos.iter().map(|exo| {
                                    html! {
                                        <option key={exo.clone().uuid} value={exo.clone().id.to_string()}>{exo.clone().name}</option>
                                    }
                                }).collect::<Html>()
                            }
                         </select>
                    </div>

                    <div class="mt-4">
                        <button type="submit" class="btn btn-primary">{"Submit"}</button>
                    </div>
                </form>
            </div>

            <BottomNav on_home_page=false on_exo_page=false />
        </body>
    }
}
