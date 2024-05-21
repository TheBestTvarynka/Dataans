use leptos::*;

#[component]
pub fn Confirm<Confirm, Cancel>(message: String, on_confirm: Confirm, on_cancel: Cancel) -> impl IntoView
where
    Confirm: Fn() -> () + 'static,
    Cancel: Fn() -> () + 'static,
{
    view! {
        <div class="confirm-page">
            <div class="confirm-window">
                <span>{message}</span>
                <div class="confirm-actions">
                    <button on:click=move |_| on_cancel() class="confirm-action-button confirm-cancel-button">
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
