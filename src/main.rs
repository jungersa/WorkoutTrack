mod components;
mod types;
use components::pages::home::Home;

fn main() {
    yew::Renderer::<Home>::new().render();
}
