use super::{bindings, colours};
use implicit_clone::{
    unsync::{IArray, IString},
    ImplicitClone,
};
use serde::{Deserialize, Serialize};
use web_sys::Element;
use yew::{html, Component, Context, Html, NodeRef, Properties};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct XYDataset {
    labels: IArray<IString>,
    data: IArray<XYData>,
}
impl ImplicitClone for XYDataset {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct XYData {
    x: u64,
    y: u64,
}
impl ImplicitClone for XYData {}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct Data {
    data: IArray<XYDataset>,
}
impl ImplicitClone for Data {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub data: IArray<XYDataset>,
    pub x_label: Option<IString>,
    pub y_label: Option<IString>,
    pub title: Option<String>,
}

pub struct XY {
    svg_ref: NodeRef,
}

impl Component for XY {
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
            data: Data { data: data.clone() },
            options: bindings::Options {
                background_color: colours::default_background(),
                legend_position: Some(bindings::PositionType::UpRight),
                y_tick_count: None,
                inner_radius: None,
                data_colors: colours::generate_unique_colors(255),
            },
        };

        let svg_element = self.svg_ref.cast::<Element>().unwrap();
        let conf = serde_wasm_bindgen::to_value(&chart_options).unwrap();
        bindings::xkcd_xy(&svg_element, &conf);
    }
}
