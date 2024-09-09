use leptos::*;

use super::app_info_window::AppInfoWindow;
use crate::common::Modal;

#[component]
pub fn AppInfo() -> impl IntoView {
    let (show_window, set_show_window) = create_signal(false);

    view! {
        <div style="display: inline-flex; width: 100%; justify-content: center; margin-bottom: 0.2em;">
            <button class="button_cancel" on:click=move |_| set_show_window.set(true)>{format!("v.{}", env!("CARGO_PKG_VERSION"))}</button>
        </div>
        <Show when=move || show_window.get()>
            <Modal>
                <AppInfoWindow close=move |_| set_show_window.set(false) />
            </Modal>
        </Show>
    }
}
