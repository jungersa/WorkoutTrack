use crate::types::Exo as ExoType;
use yew::prelude::*;

#[function_component(Exo)]
pub fn workout(props: &ExoType) -> Html {
    html! {
        <div class="border rounded p-2">
            <h3 class="text-md font-semibold mb-2">{ props.exopredef_id }</h3>
            <p class="mb-1">{"Sets: "} { props.reps_rep }</p>
            <p class="mb-1">{"Reps: "}{ props.reps_exo }</p>
            <p class="mb-1">{"Weight: "} { props.poids }</p>
        </div>
    }
}
