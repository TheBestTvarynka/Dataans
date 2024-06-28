use leptos::web_sys::KeyboardEvent;
use leptos::*;

#[component]
pub fn TextArea(
    text: ReadSignal<String>,
    #[prop(into)] set_text: Callback<String, ()>,
    #[prop(into)] key_down: Callback<KeyboardEvent, ()>,
) -> impl IntoView {
    view! {
        <div class="resizable-textarea">
            <div class="resizable-textarea-text-container">
                <span class="resizable-textarea-text">{move || format!("{}\n ", text.get())}</span>
            </div>
            <textarea
                type="text"
                placeholder="Type a note..."
                class="resizable-textarea-textarea"
                on:input=move |ev| set_text.call(event_target_value(&ev))
                on:keydown=move |ev| key_down.call(ev)
                prop:value=move || text.get()
            >
                {text.get_untracked()}
            </textarea>
        </div>
    }
}
