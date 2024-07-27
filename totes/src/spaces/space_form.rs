use common::space::{OwnedSpace, Space, UpdateSpace};
use leptos::*;
use leptos_hotkeys::use_hotkeys;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::backend::gen_avatar;
use crate::backend::spaces::{create_space, list_spaces, update_space};

const INPUT_ELEM_ID: &str = "space-form";

#[component]
pub fn SpaceForm(
    space: Option<OwnedSpace>,
    #[prop(into)] on_cancel: Callback<(), ()>,
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
) -> impl IntoView {
    let (space_name, set_space_name) = create_signal(space.as_ref().map(|s| s.name.to_string()).unwrap_or_default());
    let (avatar_path, set_avatar_path) = create_signal(
        space
            .as_ref()
            .map(|s| s.avatar.to_string())
            .unwrap_or_else(|| "/public/default_space_avatar.png".to_string()),
    );

    let generate_avatar = create_action(move |_: &()| async move {
        set_avatar_path.set(gen_avatar().await);
    });

    let id = space.as_ref().map(|s| s.id);
    let create_space = create_action(move |_: &()| {
        let name = space_name.get();
        let avatar = avatar_path.get();

        let action = async move {
            if let Some(id) = id {
                update_space(UpdateSpace {
                    id,
                    name: name.into(),
                    avatar: avatar.into(),
                })
                .await
                .expect("Space updating should not fail");
            } else {
                create_space(Space {
                    id: Uuid::new_v4().into(),
                    name: name.into(),
                    created_at: OffsetDateTime::now_utc().into(),
                    avatar: avatar.into(),
                })
                .await
                .expect("Space creation should not fail");
            }
        };

        async move {
            action.await;
            set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
            on_cancel.call(());
        }
    });

    use_hotkeys!(("Escape") => move |_| on_cancel.call(()));
    use_hotkeys!(("Enter") => move |_| create_space.dispatch(()));

    view! {
        <div class="create-space-window" on:load=move |_| info!("on_load")>
            {if space.is_some() {
                view! { <span class="create-space-title">"Update space"</span> }
            } else {
                view! { <span class="create-space-title">"Create space"</span> }
            }}
            <div class="create-space-avatar">
                <img class="create-space-avatar-img" src=avatar_path />
                <div style="align-self: center">
                    <button class="tool" title="Regenerate avatar" on:click=move |_| generate_avatar.dispatch(())>
                        <img alt="regenerate-avatar" src="/public/icons/refresh.svg" />
                    </button>
                </div>
            </div>
            <input
                id=INPUT_ELEM_ID
                type="text"
                placeholder="Space name"
                class="input"
                on:input=move |ev| set_space_name.set(event_target_value(&ev))
                prop:value=space_name
            />
            <div class="create-space-buttons">
                <button
                    class="button_cancel"
                    title="Cancel space creation"
                    on:click=move |_| on_cancel.call(())
                >
                    "Cancel"
                </button>
                <button
                    class="button_ok"
                    title="Create space"
                    on:click=move |_| create_space.dispatch(())
                >
                    {if space.is_some() { "Update" } else { "Create" }}
                </button>
            </div>
        </div>
    }
}
