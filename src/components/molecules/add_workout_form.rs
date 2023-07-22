use yew::prelude::*;

use crate::components::atoms::date_time_input::DateTimeInput;
use crate::components::atoms::text_input::TextInput;
use crate::types::WorkoutCreation;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<WorkoutCreation>,
}

#[function_component(AddWorkoutForm)]
pub fn add_workout_form(props: &Props) -> Html {
    let state: UseStateHandle<WorkoutCreation> = use_state(WorkoutCreation::default);

    let title_changed = {
        let state = state.clone();

        Callback::from(move |title| {
            state.set(WorkoutCreation {
                title,
                ..(*state).clone()
            });
        })
    };

    let date_changed = {
        let state = state.clone();

        Callback::from(move |date| {
            state.set(WorkoutCreation {
                date,
                ..(*state).clone()
            });
        })
    };

    let onsubmit = {
        let form_onsubmit = props.onsubmit.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            form_onsubmit.emit((*state).clone());
        })
    };

    html! {
    <form class="mb-4 rounded bg-white px-8 pb-8 pt-6 shadow-md" {onsubmit}>
        <div class="mb-4">
            <label class="mb-1 block font-bold text-gray-700 text-left" for="title"> {"Title"} </label>
                <TextInput
                    name="title"
                    placeholder="Title"
                    handle_change={title_changed} />
        </div>
        <div class="mb-4">
        <label class="mb-1 block font-bold text-gray-700 text-left" for="work_date"> {"Work Date"} </label>
            <DateTimeInput
                name="work_date"
                handle_change={date_changed} />
        </div>
        <div class="flex items-center justify-between">
            <button class="focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none" type="submit">{"Submit"}</button>
        </div>
    </form>
    }
}
