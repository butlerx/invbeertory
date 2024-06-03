use crate::formatting;
use implicit_clone::unsync::{IMap, IString};
use yew::{function_component, html, Children, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Deck)]
pub fn deck(props: &Props) -> Html {
    html! {
        <div class="deck">{ props.children.clone() }</div>
    }
}

#[derive(Properties, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub title: Option<IString>,
    #[prop_or_default]
    pub meta: IMap<IString, IString>,
    pub children: Children,
}

#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let CardProps {
        title,
        meta,
        children,
    } = props;

    let meta_html = if meta.is_empty() {
        html! {}
    } else {
        html! {
            <div class="metadata">
                { for meta.iter().map(|(k, v)| html! {
                    <>
                        <span class="key">{formatting::capitalise(&k)}</span>
                        { ": " }
                        <span class="val">{ v }</span>
                        <br/>
                    </>
                })}
            </div>
        }
    };

    let title_html = match title {
        Some(title) if !title.is_empty() => html! {
            <h1 class="headline">{ title }</h1>
        },
        _ => html! {},
    };

    html! {
        <div class="single">
            {meta_html}
            {title_html}
            <section class="body">{ children.clone() }</section>
        </div>
    }
}
