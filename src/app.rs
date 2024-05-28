use crate::components::{hero::Hero, layout::Layout};
use crate::pages::{home, page_not_found};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/stock")]
    Stock,
    #[at("/history")]
    History,
    #[at("/graphs")]
    Graphs,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        _ => home(),
        Route::NotFound => page_not_found(),
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
