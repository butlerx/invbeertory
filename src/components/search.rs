use crate::{
    app::Route,
    storage::{Beer, Stock},
};
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

            let search_results = stock.search(&query_value).iter().collect::<IArray<_>>();
            results.set(search_results);
        })
    };

    html! {
        <div>
            <input
                type="text"
                name="search"
                aria-label="Search"
                placeholder="What beer you looking for?"
                class="search-box"
                oninput={oninput}
            />
            <ul class="search-results">
                { for beers.iter().map(|beer| html! {
                    <li>
                        <Link<Route> to={Route::Beer{
                            brewery: beer.brewery.to_lowercase().replace(" ", "_").into(),
                            year: beer.year,
                            name: beer.name.to_lowercase().replace(" ", "_").into(),
                        }} classes={classes!("path")}>{beer}</Link<Route>>
                    </li>
                })}
            </ul>
        </div>
    }
}
