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

#[function_component(History)]
pub fn history(props: &Props) -> Html {
    let Props { stock } = props;
    let title = IString::from("History");
    html! {
        <Layout title={title.clone()}>
            <Deck>
                <Card title={title.clone()}>
                    <DataGrid data={stock.beers.clone()} />
                </Card>
            </Deck>
        </Layout>
    }
}
