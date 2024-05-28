use crate::components::layout::Layout;
use yew::{classes, html, Html};

pub fn page_not_found() -> Html {
    html! {
        <Layout>
            <div class={classes!("pageNotFound")}>
                <h1>
                    <span class={classes!("base05")}>{"["}</span>
                    <span class={classes!("base08")}>{"4"}</span>
                    <span class={classes!("base09")}>{"0"}</span>
                    <span class={classes!("base0a")}>{"4"}</span>
                    <span class={classes!("base05")}>{"]"}</span>
                    <span class={classes!("base05")}>{"# _"}</span>
                </h1>
            </div>
        </Layout>
    }
}
