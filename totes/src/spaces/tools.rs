use common::space::Space;
use leptos::*;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::backend::spaces::{create_space, list_spaces};

#[component]
pub fn Tools(set_spaces: WriteSignal<Vec<Space<'static>>>) -> impl IntoView {
    let (show_input, set_show_input) = create_signal(false);
    let (space_name, set_space_name) = create_signal(String::new());

    let create_space = move || {
        let name = space_name.get();
        spawn_local(async move {
            create_space(Space {
                id: Uuid::new_v4().into(),
                name: name.into(),
                created_at: OffsetDateTime::now_utc().into(),
            })
            .await
            .expect("space creation should not fail");
            set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
            set_show_input.set(false);
        });
    };
    let key_down = move |key| {
        if key == "Enter" {
            create_space();
        } else if key == "Escape" {
            set_show_input.set(false);
        }
    };

    view! {
        <div class="tools">
            <button class="tool" title="Add a new space" on:click=move |_| set_show_input.set(true)>
                <img alt="add-space" src="/public/icons/add-space-1.png" />
            </button>
            <Show when=move || show_input.get()>
                <div class="horizontal">
                    <input
                        type="text"
                        placeholder="Space name"
                        class="input"
                        on:input=move |ev| set_space_name.set(event_target_value(&ev))
                        on:keydown=move |ev| key_down(ev.key())
                        prop.value=space_name
                    />
                    <button
                        class="button_ok"
                        title="Create space"
                        on:click=move |_| create_space()
                    >
                        "Ok"
                    </button>
                    <button
                        class="button_cancel"
                        title="Cancel space creation"
                        on:click=move |_| set_show_input.set(false)
                    >
                        "Cancel"
                    </button>
                </div>
            </Show>
        </div>
    }
}
