use leptos::*;

use crate::common::Modal;
use super::app_info_window::AppInfoWindow;

#[component]
pub fn AppInfo() -> impl IntoView {
    let (show_window, set_show_window) = create_signal(false);

    view! {
        <div>
            <button on:click=move |_| set_show_window.set(true)>{format!("v.{}", env!("CARGO_PKG_VERSION"))}</button>
            <Show when=move || show_window.get()>
                <Modal>
                    <AppInfoWindow close=move |_| set_show_window.set(false) />
                </Modal>
            </Show>
        </div>
    }
}