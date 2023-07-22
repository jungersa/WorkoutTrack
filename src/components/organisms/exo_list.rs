use yew::prelude::*;

use crate::components::molecules::exo::Exo as ExoComponent;
use crate::types::Exo;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub exos: Vec<Exo>,
}

#[function_component(ExoList)]
pub fn workout_list(props: &Props) -> Html {
    let Props { exos } = props.clone();

    html! {
        <div class="bg-slate-100 rounded shadow-md p-4">
            <h2 class="text-lg font-bold mb-2">{"Exercises"}</h2>
            {for exos.iter().map(|exo| html! {
                <ExoComponent ..exo.clone() />
            })}
        </div>
    }
}
