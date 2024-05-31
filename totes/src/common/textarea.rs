use leptos::*;

#[component]
pub fn TextArea(
    text: ReadSignal<String>,
    #[prop(into)] set_text: Callback<String, ()>,
) -> impl IntoView {
    view! {
        <div class="resizable-textarea">
            <span class="resizable-textarea-text">{move || format!("{} ", text.get())}</span>
            <textarea
                type="text"
                placeholder="Type a note..."
                class="resizable-textarea-textarea"
                on:input=move |ev| set_text.call(event_target_value(&ev))
                // on:keydown=move |ev| key_down(ev.key())
                prop.value=text
                value=text
            />
        </div>
    }
}
