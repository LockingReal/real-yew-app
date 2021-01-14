#![recursion_limit="1024"]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::eval_order_dependence)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod error;
pub mod routes;
pub mod services;
pub mod types;
pub mod components;
pub mod model;

mod app;


#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}

