use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::components::atoms::text_input::TextInput;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct WorkoutData {
    pub title: String,
    pub date: String,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<WorkoutData>,
}

#[function_component(AddWorkoutForm)]
pub fn add_workout_form(props: &Props) -> Html {
    let state = use_state(|| WorkoutData::default());

    let cloned_state = state.clone();
    let title_changed = Callback::from(move |title| {
        cloned_state.set(WorkoutData {
            title,
            ..(*cloned_state).clone()
        })
    });

    let cloned_state = state.clone();
    let date_changed = Callback::from(move |date| {
        cloned_state.set(WorkoutData {
            date,
            ..(*cloned_state).clone()
        })
    });

    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state;
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        form_onsubmit.emit((*cloned_state).clone());
    });

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
            <TextInput
                name="work_date"
                placeholder="Work Date"
                handle_change={date_changed} />
        </div>
        <div class="flex items-center justify-between">
            <button class="focus:shadow-outline rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700 focus:outline-none" type="submit">{"Submit"}</button>
        </div>
    </form>
    }
}
