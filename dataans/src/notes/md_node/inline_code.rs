use leptos::prelude::*;

#[component]
pub fn InlineCode(code: String) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let code_value = code.clone();

    view! {
        <span class="inline-code" on:click=move |_| {
            let clipboard = window().navigator().clipboard();
            let _ = clipboard.write_text(&code_value);
            toaster.toast(
                leptoaster::ToastBuilder::new("Copied!")
                    .with_level(leptoaster::ToastLevel::Success)
                    .with_position(leptoaster::ToastPosition::BottomRight),
            );
        }>{code}</span>
    }
}
