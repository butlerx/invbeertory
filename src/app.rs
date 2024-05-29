use crate::{
    pages::{Beer, Graphs, History, Home, NotFound, Stock},
    storage,
};
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
    #[at("/:brewery/:year/:name")]
    Beer {
        brewery: String,
        year: String,
        name: String,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    let stock = storage::load().expect("Failed to parse CSV");
    match routes {
        Route::Home => html! {<Home />},
        Route::Stock => html! {<Stock stock={stock}/>},
        Route::History => html! {<History stock={stock}/>},
        Route::Graphs => html! { <Graphs stock={stock}/> },
        Route::Beer {
            brewery,
            year,
            name,
        } => html! {<Beer stock={stock} brewery={brewery} year={year} name={name} />},
        Route::NotFound => html! {<NotFound />},
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
