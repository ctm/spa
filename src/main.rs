mod app;
mod lobby;
mod table;
mod table_info;
mod util;
mod window;

pub(crate) use {
    app::Route,
    lobby::Lobby,
    table::Table,
    table_info::{TableInfo, TableWindow, Tables},
    window::Window,
};

use app::App;
use js_sys::{JSON, JsString};
use wasm_bindgen::{JsCast, prelude::*};

type TableId = u8;

pub(crate) fn human(value: JsValue) -> String {
    if let Some(string) = value.as_string() {
        string
    } else if value.has_type::<js_sys::Error>() {
        let error = value.unchecked_into::<js_sys::Error>();
        <JsString as ToString>::to_string(&error.message())
    } else {
        match JSON::stringify(&value) {
            Err(_) => format!("{value:?}"),
            Ok(js_string) => <JsString as ToString>::to_string(&js_string),
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
