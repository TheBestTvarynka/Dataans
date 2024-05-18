mod space;
mod tools;

use leptos::*;

use self::space::Space;
use self::tools::Tools;
use crate::backend::spaces::list_spaces;

#[component]
pub fn Spaces() -> impl IntoView {
    let (spaces, set_spaces) = create_signal(Vec::new());

    spawn_local(async move {
        set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
    });

    view! {
        <div class="spaces-container">
            <Tools set_spaces />
            <div class="spaces">
                {move || spaces.get().iter().cloned().map(|space| view! {
                    <Space space={space} />
                }).collect_view()}
            </div>
        </div>
    }
}
