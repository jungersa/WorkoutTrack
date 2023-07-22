mod components;
mod router;
mod types;

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::{switch, Route};

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
