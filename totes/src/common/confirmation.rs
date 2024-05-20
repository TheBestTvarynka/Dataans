use leptos::*;

#[component]
pub fn Confirm<Confirm, Cancel>(
    message: String,
    on_confirm: Confirm,
    on_cancel: Cancel,
) -> impl IntoView
where
    Confirm: Fn() -> () + 'static,
    Cancel: Fn() -> () + 'static
{
    view! {
        <div class="confirm-page">
            <div class="confirm-window">
                <span>{message}</span>
                <div class="confirm-actions">
                    <button on:click=move |_| on_confirm() class="confirm-confirm-button">"Cancel"</button>
                    <button on:click=move |_| on_cancel() class="confirm-cancel-button">"Confirm"</button>
                </div>
            </div>
        </div>
    }
}
