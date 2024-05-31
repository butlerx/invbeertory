use crate::app::Route;
use crate::storage::{Beer, Stock};
use implicit_clone::unsync::{IArray, IString};
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state, Callback, Html, InputEvent, Properties,
    TargetCast,
};
use yew_router::prelude::Link;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub stock: Stock,
}

#[function_component(Search)]
pub fn search(props: &Props) -> Html {
    let Props { stock } = props;
    let query = use_state(IString::default);
    let beers = use_state(IArray::<Beer>::default);

    let oninput = {
        let query = query.clone();
        let results = beers.clone();
        let stock = stock.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let query_value = input.value();
            query.set(IString::from(query_value.clone()));

            let search_results = stock
                .search(&query_value)
                .into_iter()
                .map(|beer| beer.clone())
                .collect::<IArray<_>>();
            results.set(search_results);
        })
    };

    html! {
        <div class="searchStyles">
            <input
                type="text"
                name="search"
                aria-label="Search"
                placeholder="What beer you looking for?"
                class="searchBox"
                oninput={oninput}
            />
            <ul class="searchResults">
                { for beers.iter().map(|beer| html! {
                    <li>
                        <Link<Route> to={Route::Beer{
                            brewery: beer.brewery.clone(),
                            year: beer.year.clone(),
                            name: beer.name.clone(),
                        }} classes={classes!("path")}>{beer}</Link<Route>>
                    </li>
                })}
            </ul>
        </div>
    }
}
