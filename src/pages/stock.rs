use crate::{components::Layout, storage};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(Stock)]
pub fn stock(props: &Props) -> Html {
    html! {
        <Layout >
            {"Current Stock"}
            // <BeerTable title="Current Stock" beers={beers} />
        </Layout>
    }
}
