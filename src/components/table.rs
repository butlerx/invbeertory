use crate::{app::Route, formatting, storage::Beer};
use implicit_clone::unsync::{IArray, IString};
use web_sys::HtmlInputElement;
use yew::{
    classes, function_component, html, use_effect_with_deps, use_state, Callback, Html, InputEvent,
    Properties, TargetCast,
};
use yew_router::prelude::Link;

const HEADERS: [(&str, &str); 8] = [
    ("name", "search"),
    ("brewery", "search"),
    ("year", "number"),
    ("abv", "number"),
    ("style", "search"),
    ("size", "number"),
    ("stock", "number"),
    ("purchased", "number"),
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
    let filtered_data = use_state(|| data.clone());

    {
        let data = data.clone();
        let filter_inputs = filter_inputs.clone();
        let filtered_data = filtered_data.clone();

        use_effect_with_deps(
            move |filter_inputs| {
                let lower_filters = filter_inputs
                    .iter()
                    .map(|f| f.to_lowercase())
                    .collect::<Vec<_>>();

                let new_filtered_data = data
                    .iter()
                    .filter(|beer| {
                        HEADERS.iter().enumerate().all(|(index, &(header, _))| {
                            lower_filters[index].is_empty()
                                || beer.filter_check(header, &lower_filters[index])
                        })
                    })
                    .collect::<IArray<_>>();

                filtered_data.set(new_filtered_data);
                || ()
            },
            filter_inputs,
        );
    }

    let on_sort = {
        let sort_by = sort_by.clone();
        let ascending = ascending.clone();
        let filtered_data = filtered_data.clone();
        Callback::from(move |(index, header): (usize, &str)| {
            let current_sort = *sort_by;
            let mut new_ascending = true;
            if let Some(current_sort) = current_sort {
                if current_sort == index {
                    new_ascending = !*ascending;
                }
            }
            sort_by.set(Some(index));
            ascending.set(new_ascending);

            let mut sorted_data = filtered_data.to_vec();
            sorted_data.sort_by(|a, b| {
                if new_ascending {
                    a.compare_field(b, header)
                } else {
                    b.compare_field(a, header)
                }
            });
            filtered_data.set(sorted_data.into());
        })
    };

    let on_filter_change = {
        let filter_inputs = filter_inputs.clone();
        Callback::from(move |(index, value): (usize, String)| {
            let mut new_filters = filter_inputs.to_vec();
            new_filters[index] = value.into();
            filter_inputs.set(new_filters.into());
        })
    };

    let header_cells = HEADERS
        .iter()
        .enumerate()
        .map(|(index, &(header, input_type))| {
            let on_click = {
                let on_sort = on_sort.clone();
                Callback::from(move |_| on_sort.emit((index, header)))
            };

            let on_input = {
                let on_filter_change = on_filter_change.clone();
                Callback::from(move |e: InputEvent| {
                    let input: HtmlInputElement = e.target_unchecked_into();
                    on_filter_change.emit((index, input.value()))
                })
            };

            html! {
                <div class={classes!("grid-header-cell")}>
                    <div
                        class={classes!("col-title")}
                        onclick={on_click}
                    >{formatting::capitalise(header)}</div>
                    <input
                        type={input_type}
                        name={format!("filter-{}", header)}
                        aria-label={format!("Filter by {}", header)}
                        oninput={on_input}
                        value={filter_inputs[index].to_string()}
                        min="0"
                    />
                </div>
            }
        })
        .collect::<Html>();

    let rows = filtered_data.iter().map(|beer| {
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
    }).collect::<Html>();

    html! {
        <div class={classes!("data-grid")}>
            <div class={classes!("grid-header")}>{ header_cells }</div>
            { rows }
        </div>
    }
}
