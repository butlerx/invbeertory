mod app;
mod components;
mod formatting;
mod pages;
mod storage;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
