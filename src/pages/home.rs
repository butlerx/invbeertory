use crate::components::{Hero, Layout};
use yew::{function_component, html, Html};

const TITLE: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <Layout>
            <Hero title={TITLE} message={DESCRIPTION} />
            //<Search searchIndex={siteSearchIndex.index} />
        </Layout>
    }
}
