use crate::{types::WorkoutProps, router::Route};
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Workout)]
pub fn workout(props: &WorkoutProps) -> Html {
    let WorkoutProps {
        id,
        uuid,
        title,
        work_date,
    } = props;

    let date = work_date.format("%B %e, %Y").to_string();
    html! {
        <Link<Route> to={Route::WorkoutDetail { uuid: uuid.to_string() }}>
            <div key={uuid.to_string()} class="bg-white rounded-lg shadow-md p-6 flex flex-row justify-between items-center mx-2">
                <div class="flex flex-col">
                    <div class="text-gray-600 font-bold text-lg mb-2 text-left">{title}</div>
                    <div class="text-gray-400 text-sm text-left">{date}</div>
                </div>
                <div class="text-green-500 font-bold text-3xl">{id}</div>
            </div>
        </Link<Route>>
    }
}
