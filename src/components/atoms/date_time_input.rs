use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub name: AttrValue,
    pub handle_change: Callback<String>,
}

#[function_component(DateTimeInput)]
pub fn date_time_input(props: &Props) -> Html {
    let Props {
        name,
        handle_change,
    } = props;

    let onchange = {
        let handle_change = handle_change.clone();

        Callback::from(move |event: Event| {
            let input = event.target_dyn_into::<HtmlInputElement>();

            if let Some(input) = input {
                handle_change.emit(input.value());
            }
        })
    };

    html! {
        <input
            class="focus:shadow-outline w-full appearance-none rounded border px-3 py-2 leading-tight text-gray-700 focus:outline-none"
            type="datetime-local"
            name={name}
            id={name}
            {onchange}
        />
    }
}
