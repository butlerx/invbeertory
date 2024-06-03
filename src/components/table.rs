use crate::{app::Route, formatting, storage::Beer};
use implicit_clone::unsync::{IArray, IString};
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_state, Callback, Html, InputEvent, Properties,
    TargetCast,
};
use yew_router::prelude::Link;

const HEADERS: [&str; 8] = [
    "Name",
    "Brewery",
    "Year",
    "ABV",
    "Style",
    "Size",
    "Stock",
    "Purchased",
];

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub data: IArray<Beer>,
}

#[function_component(DataGrid)]
pub fn data_grid(props: &Props) -> Html {
    let Props { data } = props;
    let sort_by = use_state(|| None);
    let ascending = use_state(|| true);
    let filter_inputs = use_state(|| IArray::from(vec![IString::default(); HEADERS.len()]));
    let data = use_state(|| data.clone());

    let on_sort = {
        let sort_by = sort_by.clone();
        let ascending = ascending.clone();
        let data = data.clone();
        Callback::from(move |index: usize| {
            let current_sort = *sort_by;
            let mut new_ascending = true;
            if let Some(current_sort) = current_sort {
                if current_sort == index {
                    new_ascending = !*ascending;
                }
            }
            sort_by.set(Some(index));
            ascending.set(new_ascending);

            let sorted_data = data.to_vec();
            // sorted_data.sort_by(|a, b| {
            //     if new_ascending {
            //         a[index].cmp(&b[index])
            //     } else {
            //         b[index].cmp(&a[index])
            //     }
            // });
            data.set(sorted_data.into());
        })
    };

    let _on_filter_change = {
        let data = data.clone();
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |(index, value): (usize, String)| {
            let mut new_filters = filter_inputs.to_vec();
            new_filters[index] = value.into();
            filter_inputs.set(new_filters.clone().into());

            let lower_filters = new_filters
                .iter()
                .map(|f| f.to_lowercase())
                .collect::<Vec<_>>();

            let filtered_data = data
                .iter()
                .filter(|beer| {
                    lower_filters
                        .iter()
                        .enumerate()
                        .all(|(i, filter)| filter.is_empty() || beer.filter_check(i, filter))
                })
                .collect::<IArray<_>>();
            data.set(filtered_data);
        })
    };

    let header_cells = HEADERS.iter().enumerate().map(|(index, header)| {
        let on_click = {
            let on_sort = on_sort.clone();
            Callback::from(move |_| on_sort.emit(index))
        };

        let on_input = {
            //let on_filter_change = on_filter_change.clone();
            Callback::from(move |e: InputEvent| {
                let _input: HtmlInputElement = e.target_unchecked_into();
                //on_filter_change.emit((index, input.value()))
            })
        };

        html! {
            <div class={classes!("grid-header-cell")}>
                <div class={classes!("col-title")} onclick={on_click}>{header}</div>
                <input type="text" oninput={on_input} />
            </div>
        }
    });

    let rows = data.iter().map(|beer| {
        html! {
            <div class={classes!("grid-row")}>
                <div class={classes!("grid-cell")}>
                    <Link<Route> to={Route::Beer{
                        brewery: beer.brewery.to_lowercase().replace(" ", "_").into(),
                        year: beer.year,
                        name: beer.name.to_lowercase().replace(" ", "_").into(),
                    }}>{beer.name.clone()}</Link<Route>>
                </div>
                <div class={classes!("grid-cell", "left")}>{formatting::brewer::format(&beer)}</div>
                <div class={classes!("grid-cell", "center")}>{beer.year}</div>
                <div class={classes!("grid-cell", "right")}>{format!("{:.1}%", beer.abv)}</div>
                <div class={classes!("grid-cell", "left")}>{formatting::style::format(&beer)}</div>
                <div class={classes!("grid-cell", "center")}>{format!("{} ml", beer.size)}</div>
                <div class={classes!("grid-cell", "right")}>{beer.stock}</div>
                <div class={classes!("grid-cell", "right")}>{beer.purchased}</div>
            </div>
        }
    });

    html! {
        <div class={classes!("data-grid")}>
            <div class={classes!("grid-header")}>{ for header_cells }</div>
            { for rows }
        </div>
    }
}
