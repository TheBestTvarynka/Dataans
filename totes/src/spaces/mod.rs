mod space;
mod tools;

use common::space::Space as SpaceData;
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;

use self::space::Space;
use self::tools::Tools;
use crate::backend::spaces::{create_space, list_spaces};

#[component]
pub fn Spaces() -> impl IntoView {
    let (spaces, set_spaces) = create_signal(Vec::new());

    view! {
        <div class="spaces-container">
            <Tools />
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
                <button on:click=move |_| {
                    spawn_local(async move {
                        let data = create_space(SpaceData {
                            id: Uuid::new_v4().into(),
                            name: "tbt_new_created_space".into(),
                            created_at: OffsetDateTime::now_utc().into(),
                        }).await;
                        info!("{:?}", data);
                        let data = list_spaces().await;
                        info!("{:?}", data);
                        set_spaces.set(data.unwrap());
                    })
                }>"Create"</button>
            </div>
        </div>
    }
}
