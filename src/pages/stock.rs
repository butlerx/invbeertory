use crate::{
    components::{Card, DataGrid, Deck, Layout},
    storage,
};
use implicit_clone::unsync::IString;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(Stock)]
pub fn stock(props: &Props) -> Html {
    let Props { stock } = props;
    let beers = stock.current_stock();
    let title = IString::from("Current Stock");

    html! {
        <Layout title={title.clone()}>
            <Deck>
                <Card title={title.clone()}>
                    <DataGrid data={beers} />
                </Card>
            </Deck>
        </Layout>
    }
}
