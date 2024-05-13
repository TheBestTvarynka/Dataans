
#[macro_use]
extern crate log;

mod app;
mod spaces;
mod profile;
mod messages;
mod backend;

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
