use super::{bindings, colours::generate_unique_colors};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::Element;
use yew::{html, Component, Context, Html, NodeRef, Properties};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Data {
    pub datasets: Vec<Dataset>,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Dataset {
    pub data: Vec<i64>,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: HashMap<String, i64>,
    pub title: Option<String>,
}

pub struct Pie {
    svg_ref: NodeRef,
}

impl Component for Pie {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            svg_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
        <div style="width: 100%; height: auto;">
            <svg ref={self.svg_ref.clone()} />
        </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let Props { title, data } = &ctx.props();
        let chart_options = bindings::ChartOptions {
            title: title.clone(),
            x_label: None,
            y_label: None,
            data: Data {
                labels: data.keys().cloned().collect(),
                datasets: vec![Dataset {
                    data: data.values().copied().collect(),
                }],
            },
            options: bindings::Options {
                background_color: "#f2f0ec".to_string(),
                legend_position: Some(bindings::PositionType::UpRight),
                y_tick_count: None,
                inner_radius: Some(0.4),
                data_colors: generate_unique_colors(255),
            },
        };

        let svg_element = self.svg_ref.cast::<Element>().unwrap();
        let conf = serde_wasm_bindgen::to_value(&chart_options).unwrap();
        bindings::xkcd_pie(&svg_element, &conf);
    }
}
