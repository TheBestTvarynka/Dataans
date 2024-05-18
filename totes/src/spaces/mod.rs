mod space;
mod tools;

use leptos::*;

use self::space::Space;
use self::tools::Tools;
use crate::backend::spaces::list_spaces;

#[component]
pub fn Spaces() -> impl IntoView {
    let (spaces, set_spaces) = create_signal(Vec::new());

    view! {
        <div class="spaces-container">
            <Tools set_spaces />
            <div class="spaces">
                {move || spaces.get().iter().cloned().map(|space| view! {
                    <Space space={space} />
                }).collect_view()}
                <button on:click=move |_| {
                    spawn_local(async move {
                        let data = list_spaces().await;
                        info!("{:?}", data);
                        set_spaces.set(data.unwrap());
                    })
                }>"Load"</button>
            </div>
        </div>
    }
}
