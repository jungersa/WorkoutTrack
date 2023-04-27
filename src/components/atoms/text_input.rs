use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TextInputProps {
    pub name: AttrValue,
    pub placeholder: AttrValue,
    pub handle_change: Callback<String>,
}

#[function_component(TextInput)]
pub fn workout(props: &TextInputProps) -> Html {
    let TextInputProps {
        name,
        placeholder,
        handle_change,
    } = props;

    let handle_change_cloned = handle_change.clone();
    let onchange = Callback::from(move |event: Event| {
        let input = event.target_dyn_into::<HtmlInputElement>();

        if let Some(input) = input {
            handle_change_cloned.emit(input.value());
        }
    });

    html! {
        <input
            class="focus:shadow-outline w-full appearance-none rounded border px-3 py-2 leading-tight text-gray-700 focus:outline-none"
            type="text"
            name={name}
            placeholder={placeholder}
            {onchange}
        />
    }
}
