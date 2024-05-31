use crate::{
    components::{Hero, Layout, Search},
    storage,
};
use yew::{function_component, html, Html, Properties};

const TITLE: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    html! {
        <Layout>
            <Hero title={TITLE} message={DESCRIPTION} />
            <Search stock={props.stock.clone()} />
        </Layout>
    }
}
