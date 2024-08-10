#![allow(clippy::empty_docs)]

#[macro_use]
extern crate log;

mod app;
mod backend;
mod common;
mod notes;
mod spaces;
mod utils;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Info));

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
