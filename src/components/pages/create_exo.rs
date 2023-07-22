use crate::components::molecules::bottom_nav::BottomNav;
use serde::Serialize;
use wasm_bindgen::prelude::*;
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
pub fn create_exo(props: &Props) -> Html {
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
                            <option value="1">{"Test"}</option>
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
