use crate::{
    components::Card,
    formatting::{brewer, style, title},
    storage::Beer,
};
use std::collections::HashMap;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub beer: Beer,
}

#[function_component(Info)]
pub fn info(props: &Props) -> Html {
    let Props { beer } = props;
    let beer_info = format!(
        "{} by {} is a {}.",
        beer.name,
        brewer::format(beer),
        style::format(beer)
    );
    let meta = HashMap::from_iter(vec![
        ("purchased".to_string(), beer.purchased.to_string()),
        ("stock".to_string(), beer.stock.to_string()),
        ("abv".to_string(), format!("{}%", beer.abv)),
    ]);

    html! {
        <Card title={title::format(beer)} meta={meta}>{beer_info}</Card>
    }
}
