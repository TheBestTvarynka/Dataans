use leptos::*;

#[component]
pub fn Confirm<Confirm>(
    message: String,
    on_confirm: Confirm,
    #[prop(into)] on_cancel: Callback<(), ()>,
) -> impl IntoView
where
    Confirm: Fn() -> () + 'static,
{
    let key_down = move |key| {
        info!("key down: {}", key);
        if key == "Escape" {
            on_cancel.call(());
        }
    };

    view! {
        <div
            name="confirm-page-background"
            class="confirm-page"
            on:keydown=move |ev| key_down(ev.key())
            tabindex=0
        >
            <div class="confirm-window">
                <span>{message}</span>
                <div class="confirm-actions">
                    <button on:click=move |_| on_cancel.call(()) class="confirm-action-button confirm-cancel-button">
                        "Cancel"
                    </button>
                    <button on:click=move |_| on_confirm() class="confirm-action-button confirm-ok-button">
                        "Confirm"
                    </button>
                </div>
            </div>
        </div>
    }
}
