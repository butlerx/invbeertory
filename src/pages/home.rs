use crate::{app::Route, components::layout::Layout};
use yew::{classes, html, Html};
use yew_router::prelude::Link;

pub fn home() -> Html {
    html! {
        <Layout>
            //<SEO title="Invbeertory" />
            <Hero title={"invbeertory"} message={"Simple beer inventory display"} />
            //<Search searchIndex={siteSearchIndex.index} />
        </Layout>
    }
}
