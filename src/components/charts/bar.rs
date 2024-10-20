use super::{bindings, colours};
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};
use serde::{Deserialize, Serialize};
use web_sys::Element;
use yew::{html, Component, Context, Html, NodeRef, Properties};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Data {
    pub datasets: IArray<Dataset>,
    pub labels: IArray<IString>,
}

impl ImplicitClone for Data {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Dataset {
    pub data: IArray<i64>,
}

impl ImplicitClone for Dataset {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: IArray<(IString, i64)>,
    pub x_label: Option<IString>,
    pub y_label: Option<IString>,
    pub title: Option<String>,
}

pub struct Bar {
    svg_ref: NodeRef,
}

impl Component for Bar {
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
        let Props {
            title,
            x_label,
            y_label,
            data,
        } = &ctx.props();
        let chart_options = bindings::ChartOptions {
            title: title.clone(),
            x_label: x_label.clone(),
            y_label: y_label.clone(),
            data: Data {
                labels: data.iter().map(|(label, _)| label.clone()).collect(),
                datasets: vec![Dataset {
                    data: data.iter().map(|(_, value)| value).collect(),
                }]
                .into(),
            },
            options: bindings::Options {
                background_color: colours::default_background(),
                y_tick_count: Some(5),
                legend_position: None,
                inner_radius: None,
                data_colors: colours::generate_unique_colors(255),
            },
        };

        let svg_element = self.svg_ref.cast::<Element>().unwrap();
        let conf = serde_wasm_bindgen::to_value(&chart_options).unwrap();
        bindings::xkcd_bar(&svg_element, &conf);
    }
}
