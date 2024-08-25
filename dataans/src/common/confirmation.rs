use leptos::*;
use leptos_hotkeys::use_hotkeys;

#[component]
pub fn Confirm(
    message: String,
    #[prop(into)] on_confirm: Callback<(), ()>,
    #[prop(into)] on_cancel: Callback<(), ()>,
) -> impl IntoView {
    use_hotkeys!(("Escape") => move |_| on_cancel.call(()));
    use_hotkeys!(("Enter") => move |_| on_confirm.call(()));

    view! {
        <div
            name="confirm-page-background"
            class="confirm-page"
            tabindex=0
        >
            <div class="confirm-window">
                <span>{message}</span>
                <div class="confirm-actions">
                    <button on:click=move |_| on_cancel.call(()) class="confirm-action-button confirm-cancel-button">
                        "Cancel"
                    </button>
                    <button on:click=move |_| on_confirm.call(()) class="confirm-action-button confirm-ok-button">
                        "Confirm"
                    </button>
                </div>
                <span style="font-size: 0.65em; width: 100%">"(`Esc` to cancel. `Enter` to confirm)"</span>
            </div>
        </div>
    }
}
