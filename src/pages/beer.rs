use crate::{
    components::{Card, Deck, Layout},
    formatting::{brewer, style, title},
    storage,
};
use implicit_clone::unsync::{IMap, IString};
use yew::{function_component, html, Html, Properties};

fn beer_meta(beer: &storage::Beer) -> IMap<IString, IString> {
    IMap::from_iter(vec![
        (
            IString::from("purchased"),
            IString::from(beer.purchased.to_string()),
        ),
        (
            IString::from("stock"),
            IString::from(beer.stock.to_string()),
        ),
        (
            IString::from("abv"),
            IString::from(format!("{}%", beer.abv)),
        ),
    ])
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
    pub brewery: IString,
    pub year: i16,
    pub name: IString,
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
        <Layout title={name.clone()}>
            <Deck> {
                match stock.clone().find_beer(brewery, *year, name) {
                    Some(beer) => html! {
                        <Card title={title::format(&beer)} meta={beer_meta(&beer)}>{
                            format!(
                                "{} by {} is a {}.",
                                beer.name, brewer::format(&beer), style::format(&beer)
                            )
                        }</Card>
                    },
                    None => html! {<Card><p>{"Beer not found"}</p></Card>},
                }
            } </Deck>
        </Layout>
    }
}
