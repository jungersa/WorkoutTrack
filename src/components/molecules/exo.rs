use crate::types::{ApiCall, Exo as ExoType, PredefExo};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[function_component(Exo)]
pub fn workout(props: &ExoType) -> Html {
    let predefexo = use_state_eq(PredefExo::default);

    {
        let predefexo = predefexo.clone();
        let props = props.clone();
        spawn_local(async move {
            let args = to_value(&ApiCall {
                id: props.exopredef_id,
            })
            .expect("Couldn't transform the ApiCall to JsValue");
            let predefexo_data = invoke("get_predefined_exercice", args)
                .await
                .expect("failed to get predefexo");

            let predefexo_data: PredefExo =
                from_value(predefexo_data).expect("Couldn't transform the data to PredefExo");
            predefexo.set(predefexo_data);
        });
    };

    html! {
        <div class="border rounded p-2">
            <h3 class="text-md font-semibold mb-2">{ predefexo.name.clone() }</h3>
            <p class="mb-1">{"Sets: "} { props.reps_rep }</p>
            <p class="mb-1">{"Reps: "}{ props.reps_exo }</p>
            <p class="mb-1">{"Weight: "} { props.poids }</p>
        </div>
    }
}
