mod components;
mod types;
mod router;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{Route, switch};

fn main() {
    yew::Renderer::<Main>::new().render();
}
#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}