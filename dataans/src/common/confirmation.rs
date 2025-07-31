use leptos::callback::Callback;
use leptos::ev::keydown;
use leptos::prelude::*;
use leptos_use::use_event_listener;

#[component]
pub fn Confirm(
    message: String,
    #[prop(into)] on_confirm: Callback<(), ()>,
    #[prop(into)] on_cancel: Callback<(), ()>,
) -> impl IntoView {
    let confirm_window_element = NodeRef::new();

    let _ = use_event_listener(confirm_window_element, keydown, move |ev| {
        let key = ev.key();

        if key == "Escape" {
            ev.prevent_default();
            on_cancel.run(());
        } else if key == "Enter" {
            ev.prevent_default();
            on_confirm.run(());
        }
    });

    view! {
        <div
            id="confirm-page-background"
            class="confirm-page"
            tabindex=0
            node_ref=confirm_window_element
            autofocus
        >
            <div class="confirm-window">
                <span>{message}</span>
                <div class="confirm-actions">
                    <button on:click=move |_| on_cancel.run(()) class="confirm-action-button confirm-cancel-button">
                        "Cancel"
                    </button>
                    <button on:click=move |_| on_confirm.run(()) class="confirm-action-button confirm-ok-button">
                        "Confirm"
                    </button>
                </div>
                <span style="font-size: 0.65em; width: 100%">"(`Esc` to cancel. `Enter` to confirm)"</span>
            </div>
        </div>
    }
}
