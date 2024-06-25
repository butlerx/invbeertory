use crate::app::Route;
use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(Header)]
pub fn header() -> Html {
    let path_class = "path";

    html! {
        <header>
            <div class={classes!("container", "clearfix")}>
                <Link<Route> to={Route::Home} classes={classes!(path_class)}>{ "invbeertory" } </Link<Route>>
                <span class={classes!("caret")}>{ " #_" }</span>
                <div class={classes!("right")}>
                    <Link<Route> to={Route::Stock} classes={classes!(path_class)}>{"Current Stock"}</Link<Route>>
                    {" | "}
                    <Link<Route> to={Route::History} classes={classes!(path_class)}>{ "History" }</Link<Route>>
                    {" | "}
                    <Link<Route> to={Route::Graphs} classes={classes!(path_class)}>{ "Graphs" }</Link<Route>>
                </div>
            </div>
        </header>
    }
}
