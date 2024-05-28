use yew::{classes, function_component, html, Html, Properties};

// unit is a number between 0 and 15
// it represents a base16 digit
fn base(unit: usize) -> html::Classes {
    if unit > 9 {
        // convert 10 to a, 11 to b, etc
        let unit_char = (unit - 8 + b'a' as usize) as u8 as char;
        return classes!(format!("base0{}", unit_char));
    }

    classes!(format!("base{:02}", unit + 2))
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub message: String,
}

#[function_component(Hero)]
pub fn hero(props: &Props) -> Html {
    let Props { title, message } = props;
    html! {
      <>
        <h1 class={classes!("siteTitle")}>
          <span class={base(5)}>{"["}</span>
            { for title.chars().enumerate().map(|(i, letter)| {
                html! { <span class={base(i)}>{letter}</span> }
            })}
          <span class={base(5)}>{"]"}</span>
          <span class={base(5)}>{"#_"}</span>
        </h1>
        <div class={classes!("heroLogo")} />
        <div style="text-align: center">{message}</div>
      </>
    }
}
