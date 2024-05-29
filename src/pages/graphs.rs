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
                    data={storage::purchases_by_brewery(&stock.beers)} />
              </Card>
              <Card>
                <charts::Bar
                    title="ABV in stock"
                    data={storage::stock_by_abv(&stock.beers)}
                />
              </Card>
              <Card>
                <charts::Pie
                    title="Beers by Style"
                    data={storage::purchases_by_style(&stock.beers)}
                />
              </Card>
              <Card>
                <charts::StackedBar
                    title="Brewery by Style"
                    data={storage::brewery_by_style(&stock.beers)}
                />
              </Card>
            </Deck>
        </Layout>
    }
}
