use std::collections::HashMap;
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct DeckProps {
    pub children: Children,
}

#[function_component(Deck)]
pub fn deck(props: &DeckProps) -> Html {
    html! {
        <div class="deck">{ props.children.clone() }</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub meta: HashMap<String, String>,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        title,
        meta,
        children,
    } = props;

    let title_case = |word: &str| -> String {
        let mut chars = word.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    };

    html! {
        <div class="single">
            {
                if !meta.is_empty() {
                    html! {
                        <div class="metadata">
                            { for meta.iter().map(|(k, v)| html! {
                                <>
                                    <span class="key">{title_case(k)}</span>
                                    { ": " }
                                    <span class="val">{ v }</span>
                                    <br/>
                                </>
                            })}
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            {
                match title {
                    Some(title) if !title.is_empty() => html! {
                        <h2 class="title">{ title }</h2>
                    },
                    _ => html! {},
                }
            }
            <section class="body">{ children.clone() }</section>
        </div>
    }
}
