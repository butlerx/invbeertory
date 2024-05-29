use crate::{
    components::{Card, Deck, Info, Layout},
    storage,
};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
    pub brewery: String,
    pub year: String,
    pub name: String,
}

#[function_component(Beer)]
pub fn beer(props: &Props) -> Html {
    let Props {
        stock,
        brewery,
        year,
        name,
    } = props;

    html! {
        <Layout title={name.to_string()}>
            <Deck>
            {
                match storage::find_beer(&stock.beers, brewery, year, name) {
                    Some(beer) => html! {<Info beer={beer.clone()} />},
                    None => html! {<Card><p>{"Beer not found"}</p></Card>},
                }
            }
            </Deck>
        </Layout>
    }
}
