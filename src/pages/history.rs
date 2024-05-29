use crate::{components::Layout, storage};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(History)]
pub fn history(_props: &Props) -> Html {
    html! {
        <Layout title="History">
            {"History"}
            // <BeerTable title="History" beers={beers} />
        </Layout>
    }
}
