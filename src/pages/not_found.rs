use crate::{
    app::Route,
    components::{Card, Deck, Layout},
};
use yew::{classes, function_component, html, Html};
use yew_router::prelude::Link;

#[function_component(NotFound)]
pub fn page_not_found() -> Html {
    html! {
        <Layout title="404: Not found" >
            <div class={classes!("pageNotFound")}>
                <h1>
                    <span class={classes!("base05")}>{"["}</span>
                    <span class={classes!("base08")}>{"4"}</span>
                    <span class={classes!("base09")}>{"0"}</span>
                    <span class={classes!("base0a")}>{"4"}</span>
                    <span class={classes!("base05")}>{"]"}</span>
                    <span class={classes!("base05")}>{"# _"}</span>
                </h1>
                 <Deck>
                    <Card>
                        <p>{"You just hit a route that doesn&#39;t exist... the sadness."}</p>
                        <p>{"Please head back "}<Link<Route> to={Route::Home} classes={classes!("path")}>{"home"}</Link<Route>>{"."}</p>
                     </Card>
                </Deck>
            </div>
        </Layout>
    }
}
