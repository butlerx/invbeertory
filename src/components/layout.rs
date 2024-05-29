use crate::components::header::Header;
use chrono::Datelike;

use web_sys::window;
use yew::{classes, function_component, html, Children, Html, Properties};

const TITLE: &str = env!("CARGO_PKG_NAME");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn get_month(month_index: u32) -> &'static str {
    match month_index {
        0 => "January",
        1 => "February",
        2 => "March",
        3 => "April",
        4 => "May",
        5 => "June",
        6 => "July",
        7 => "August",
        8 => "September",
        9 => "October",
        10 => "November",
        11 => "December",
        _ => "Unknown",
    }
}

fn update_title(title: &Option<String>) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            let site_title = capitalise(TITLE);
            let title = if let Some(title) = &title {
                format!("{title} | {site_title}")
            } else {
                site_title
            };
            document.set_title(&title);
        }
    }
}

fn set_meta_tags(title: &Option<String>) {
    let window = window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let head = document.head().expect("should have a head in document");

    let title = match title {
        Some(t) => t.to_string(),
        None => capitalise(TITLE),
    };

    let meta_tags = [
        ("name", "description", DESCRIPTION),
        ("property", "og:title", &title),
        ("property", "og:description", DESCRIPTION),
        ("name", "twitter:creator", AUTHORS),
        ("name", "twitter:title", &title),
        ("name", "twitter:description", DESCRIPTION),
    ];

    for (attr, name, content) in meta_tags {
        let meta_element = document.create_element("meta").unwrap();
        meta_element.set_attribute(attr, name).unwrap();
        meta_element.set_attribute("content", content).unwrap();
        head.append_child(&meta_element).unwrap();
    }
}

fn capitalise(s: &str) -> String {
    let mut c = s.chars();
    let f = c.next().unwrap();
    f.to_uppercase().collect::<String>() + c.as_str()
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub title: Option<String>,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let current_date = chrono::Utc::now();
    let Props { title, children } = props;
    update_title(title);
    set_meta_tags(title);
    html! {
        <>
            <Header />
            <main class={classes!("container")}>{ children.clone() }</main>
            <footer>
                <div class={classes!("container")}>
                    <span>
                        { format!( "© {}, {} {} - ", get_month(current_date.month0()), current_date.year(), AUTHORS)}
                        <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">
                            { " CC BY 4.0" }
                        </a>
                    </span>
                </div>
            </footer>
        </>
    }
}
