use crate::{
    components::{Layout, Search},
    storage,
};
use yew::{classes, function_component, html, Html, Properties};

const TITLE: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// unit is a number between 0 and 15
// it represents a base16 digit
fn base(unit: usize) -> html::Classes {
    if unit > 9 {
        // convert 10 to a, 11 to b, etc
        let unit_char = (unit - 8 + b'a' as usize) as u8 as char;
        return classes!(format!("base0{unit_char}"));
    }

    classes!(format!("base{:02}", unit + 2))
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub stock: storage::Stock,
}

#[function_component(Home)]
pub fn home(props: &Props) -> Html {
    html! {
        <Layout>
            <h1 class={classes!("site-title")}>
              <span class={base(5)}>{"["}</span>
                { for TITLE.chars().enumerate().map(|(i, letter)| {
                    html! { <span class={base(i)}>{letter}</span> }
                })}
              <span class={base(5)}>{"]"}</span>
              <span class={base(5)}>{"#_"}</span>
            </h1>
            <div class={classes!("herologo")} />
            <div style="text-align: center">{DESCRIPTION}</div>
            <Search stock={props.stock.clone()} />
        </Layout>
    }
}
