use crate::{
    pages::{Beer, Graphs, History as HistoryPage, Home, NotFound, Stock},
    storage,
};
use implicit_clone::unsync::IString;
use std::collections::HashMap;
use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
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
        brewery: IString,
        year: i16,
        name: IString,
    },
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    let stock = storage::Stock::load().expect("Failed to parse CSV");
    match routes {
        Route::Home => html! {<Home stock={stock} />},
        Route::Stock => html! {<Stock stock={stock}/>},
        Route::History => html! {<HistoryPage stock={stock}/>},
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

#[derive(Properties, PartialEq, Eq, Debug)]
pub struct ServerAppProps {
    pub url: AttrValue,
    pub queries: HashMap<String, String>,
}

#[function_component(ServerApp)]
pub fn server_app(props: &ServerAppProps) -> Html {
    let history = AnyHistory::from(MemoryHistory::new());
    history
        .push_with_query(&*props.url, &props.queries)
        .unwrap();

    html! {
        <Router history={history}>
            <Switch<Route> render={switch} />
        </Router>
    }
}
