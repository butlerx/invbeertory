use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::Element;

#[wasm_bindgen(module = "/src/components/charts/xkcd.js")]
extern "C" {
    #[wasm_bindgen(js_name = "bar")]
    pub fn xkcd_bar(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "line")]
    pub fn xkcd_line(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "pie")]
    pub fn xkcd_pie(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "radar")]
    pub fn xkcd_radar(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "stackedBar")]
    pub fn xkcd_stacked_bar(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "xy")]
    pub fn xkcd_xy(canvas: &Element, options: &JsValue);

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub background_color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_tick_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_position: Option<PositionType>,
    pub data_colors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_radius: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOptions<T: Serialize> {
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_label: Option<String>,
    pub data: T,
    pub options: Options,
}

#[repr(u8)]
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
pub enum PositionType {
    UpLeft = 1,
    UpRight = 2,
    DownLeft = 3,
    DownRight = 4,
}
