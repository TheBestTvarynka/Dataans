use leptos::*;

use crate::backend::parse_code;

#[component]
pub fn CodeBlock(code: String, lang: String) -> impl IntoView {
    let language = lang.clone();
    let code_value = code.clone();
    let highlighted_code = create_resource(
        || (),
        move |_| {
            let code_value = code_value.clone();
            let lang = language.clone();
            async move { parse_code(&lang, &code_value).await }
        },
    );

    let code_value = code.clone();

    view! {
        <div class="note-code-block">
            <div class="note-code-block-meta">
                <i>{lang}</i>
                <button on:click=move |_| {
                    let clipboard = window().navigator().clipboard();
                    let _ = clipboard.write_text(&code_value);
                }>"Copy"</button>
            </div>
            <Suspense
                fallback=move || view! { <span>"Parsing code...."</span> }
            >
                {move || highlighted_code.get()
                    .map(|inner_html| view! {
                        <div class="code-block-wrapper" inner_html=inner_html />
                    })}
            </Suspense>
        </div>
    }
}
