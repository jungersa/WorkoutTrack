use chrono::NaiveDateTime;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct WorkoutProps {
    pub id: i32,
    pub uuid: AttrValue,
    pub title: AttrValue,
    pub work_date: NaiveDateTime,
}

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
        <div class="bg-white rounded-lg shadow-md p-6 flex flex-row justify-between items-center mx-2">
            <div class="flex flex-col">
                <div class="text-gray-600 font-bold text-lg mb-2 text-left">{title}</div>
                <div class="text-gray-400 text-sm text-left">{date}</div>
            </div>
            <div class="text-green-500 font-bold text-3xl">{"$99"}</div>
        </div>
    }
}
