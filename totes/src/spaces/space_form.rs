use common::space::{OwnedSpace, Space, UpdateSpace};
use leptos::{
    component, create_signal, event_target_value, spawn_local, view, Callable, Callback, IntoView, SignalGet,
    SignalSet, SignalSetter,
};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::backend::gen_avatar;
use crate::backend::spaces::{create_space, list_spaces, update_space};

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

    let generate_avatar = move || {
        spawn_local(async move {
            set_avatar_path.set(gen_avatar().await);
        });
    };

    let id = space.as_ref().map(|s| s.id);
    let create_space = move || {
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

        spawn_local(async move {
            action.await;
            set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
            on_cancel.call(());
        });
    };

    let key_down = move |key| {
        if key == "Enter" {
            create_space();
        } else if key == "Escape" {
            on_cancel.call(());
        }
    };

    view! {
        <div class="create-space-window">
            <span class="create-space-title">"Create space"</span>
            <div class="create-space-avatar">
                <img class="create-space-avatar-img" src=move || avatar_path />
                <div style="align-self: center">
                    <button class="tool" title="Regenerate avatar" on:click=move |_| generate_avatar()>
                        <img alt="regenerate-avatar" src="/public/icons/refresh.svg" />
                    </button>
                </div>
            </div>
            <input
                type="text"
                placeholder="Space name"
                class="input"
                on:input=move |ev| set_space_name.set(event_target_value(&ev))
                on:keydown=move |ev| key_down(ev.key())
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
                    on:click=move |_| create_space()
                >
                    {if space.is_some() { "Update" } else { "Create" }}
                </button>
            </div>
        </div>
    }
}
