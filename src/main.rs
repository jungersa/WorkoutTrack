mod components;

use components::pages::home::Home;

fn main() {
    yew::Renderer::<Home>::new().render();
}
