use leptos::html;
use leptos::prelude::*;

use crate::backend::parse_code;

#[component]
pub fn CodeBlock(code: String, lang: String) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let language = lang.clone();
    let code_value = code.clone();
    let highlighted_code = LocalResource::new(move || {
        let code_value = code_value.clone();
        let lang = language.clone();
        async move { parse_code(&lang, &code_value).await.unwrap_or(code_value.clone()) }
    });

    let code_container_ref = NodeRef::<html::Div>::new();

    Effect::new(move || {
        highlighted_code.get();
        if let Some(code_container) = code_container_ref.get() {
            let _ = code_container.scroll_height();
        }
    });

    let code_value = code.clone();

    view! {
        <div class="note-code-block">
            <div class="note-code-block-meta-container">
                <button
                    class="code-block-tool"
                    title="copy code"
                    on:click=move |_| {
                        let clipboard = window().navigator().clipboard();
                        let _ = clipboard.write_text(&code_value);
                        toaster.toast(
                            leptoaster::ToastBuilder::new("Copied!")
                                .with_level(leptoaster::ToastLevel::Success)
                                .with_position(leptoaster::ToastPosition::BottomRight),
                        );
                    }
                >
                    <img alt="copy code" src="/public/icons/copy-dark.png" />
                </button>
            </div>
            <div class="code-block-wrapper" node_ref=code_container_ref>
                <Suspense
                    fallback=move || view! { <span>"Parsing code...."</span> }
                >
                    {move || highlighted_code.get()
                        .map(|inner_html| {
                            let mut code = Dom::create_element_from_html(inner_html.into());
                            code.mount(&code_container_ref.get().expect("code container should be mounted"), None);

                            view! {
                                <span />
                            }
                        })}
                </Suspense>
            </div>
        </div>
    }
}
