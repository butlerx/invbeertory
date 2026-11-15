use implicit_clone::unsync::{IArray, IString};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use wasm_bindgen::{prelude::*, JsValue};
use web_sys::Element;

#[wasm_bindgen(module = "/src/components/charts/xkcd.js")]
extern "C" {
    #[wasm_bindgen(js_name = "bar")]
    #[allow(
        unsafe_code,
        reason = "wasm-bindgen FFI declarations are unsafe functions"
    )]
    pub fn xkcd_bar(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "pie")]
    #[allow(
        unsafe_code,
        reason = "wasm-bindgen FFI declarations are unsafe functions"
    )]
    pub fn xkcd_pie(canvas: &Element, options: &JsValue);
    #[wasm_bindgen(js_name = "stackedBar")]
    #[allow(
        unsafe_code,
        reason = "wasm-bindgen FFI declarations are unsafe functions"
    )]
    pub fn xkcd_stacked_bar(canvas: &Element, options: &JsValue);

}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub background_color: IString,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_tick_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legend_position: Option<PositionType>,
    pub data_colors: IArray<IString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_radius: Option<f64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartOptions<T: Serialize> {
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_label: Option<IString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_label: Option<IString>,
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
