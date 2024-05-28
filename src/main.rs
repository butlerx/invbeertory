mod app;
mod components;
mod not_found;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
