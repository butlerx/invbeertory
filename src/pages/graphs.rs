use crate::{
    components::{charts, Card, Deck, Layout},
    storage,
};
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(Graphs)]
pub fn graphs(props: &Props) -> Html {
    let Props { stock } = props;

    html! {
        <Layout title="Brewery Stats">
            <Deck>
              <Card>
                <charts::Pie
                    title="Beers by Brewery"
                    data={stock.purchases_by_brewery()} />
              </Card>
              <Card>
                <charts::Bar
                    title="ABV in stock"
                    data={stock.stock_by_abv()}
                />
              </Card>
              <Card>
                <charts::Pie
                    title="Beers by Style"
                    data={stock.purchases_by_style()}
                />
              </Card>
              <Card>
                <charts::StackedBar
                    title="Brewery by Style"
                    data={stock.brewery_by_style()}
                />
              </Card>
            </Deck>
        </Layout>
    }
}
