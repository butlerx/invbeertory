use crate::components::header::Header;
use chrono::Datelike;
use yew::{classes, function_component, html, Children, Html, Properties};

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
#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &Props) -> Html {
    let current_date = chrono::Utc::now();
    let current_month = get_month(current_date.month0());
    let author = "Cian Butler <butlerx@notthe.cloud>";
    let footer = format!("© {}, {} {} - ", current_month, current_date.year(), author);
    let children = props.children.clone();
    html! {
        <>
            <Header />
            <div class={classes!("container")}>
                <main>{ children }</main>
            </div>
            <footer>
                <div class={classes!("container")}>
                    <span>
                        {footer}
                        <a rel="license" href="http://creativecommons.org/licenses/by/4.0/">
                            { " CC BY 4.0" }
                        </a>
                    </span>
                </div>
            </footer>
        </>
    }
}
