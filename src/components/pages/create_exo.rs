use crate::{
    components::molecules::bottom_nav::BottomNav,
    types::{PredefExo, PredefExoVec},
};
use gloo_console::log;
use serde::Serialize;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Properties, Serialize, PartialEq, Clone, Debug, Eq)]
pub struct Props {
    pub uuid: String,
}

#[function_component(CreateExo)]
pub fn create_exo(_props: &Props) -> Html {
    let predefexos: UseStateHandle<Vec<PredefExo>> = use_state_eq(|| vec![]);

    {
        let predefexos = predefexos.clone();
        spawn_local(async move {
            let predefexos_data = invoke("get_predefined_exercices", JsValue::NULL)
                .await
                .expect("failed to get workouts");
            let predefexos_data: PredefExoVec =
                from_value(predefexos_data).expect("Couldn't transform the data to PredefExoVec");
            predefexos.set(predefexos_data.predefexos);
            log!("predefexos_data");
        });
    };

    html! {
        <body class="item-center">
            <div class="container mx-auto py-8">
                <h1 class="text-3xl font-bold mb-2">{"Create a new exercise"}</h1>
                <form class="bg-white p-6 rounded shadow-md">
                    <div class="mb-4">
                        <label for="reps_rep" class="block text-gray-700 font-bold mb-2">{"Number of sets:"}</label>
                        <input type="number" min="0" name="reps_rep" id="reps_rep" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true/>
                    </div>

                    <div class="mb-4">
                        <label for="reps_exo" class="block text-gray-700 font-bold mb-2">{"Number of reps:"}</label>
                        <input type="number" min="0" name="reps_exo" id="reps_exo" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true/>
                    </div>

                    <div class="mb-4">
                        <label for="poids" class="block text-gray-700 font-bold mb-2">{"Poids:"}</label>
                        <input type="number" min="0" name="poids" id="poids" class="form-input w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300"/>
                    </div>

                    <div class="mb-4">
                        <label for="exopredef_id" class="block text-gray-700 font-bold mb-2">{"Exercise:"}</label>
                        <select name="exopredef_id" id="exopredef_id" class="form-select w-full px-3 py-2 border rounded-lg shadow-sm focus:outline-none focus:ring focus:border-blue-300" required=true>
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
                        <button type="" class="btn btn-primary">{"GoBack"}</button>
                        <button type="submit" class="btn btn-primary">{"Submit"}</button>
                    </div>
                </form>
            </div>

            <BottomNav on_home_page=false on_exo_page=false />
        </body>
    }
}
