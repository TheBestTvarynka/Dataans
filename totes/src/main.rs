#![allow(clippy::empty_docs)]

#[macro_use]
extern crate log;

mod app;
mod backend;
mod notes;
mod profile;
mod spaces;

use app::*;
use leptos::*;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
