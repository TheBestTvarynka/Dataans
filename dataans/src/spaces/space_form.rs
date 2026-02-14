use common::Config;
use common::space::{Avatar, CreateSpace, OwnedSpace, UpdateSpace};
use leptos::callback::Callback;
use leptos::ev::keydown;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::use_event_listener;
use uuid::Uuid;

use crate::backend::convert_file_src;
use crate::backend::file::{gen_avatar, pick_avatar};
use crate::backend::spaces::{create_space, list_spaces, update_space};
use crate::dom::MatchKeyBinding;

const INPUT_ELEM_ID: &str = "space-form";

#[component]
pub fn SpaceForm(
    space: Option<OwnedSpace>,
    #[prop(into)] on_cancel: Callback<(), ()>,
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
    set_selected_space: Callback<(OwnedSpace,), ()>,
    #[allow(unused_variables)] config: Config,
) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (space_name, set_space_name) = signal(space.as_ref().map(|s| s.name.to_string()).unwrap_or_default());
    let (avatar, set_avatar) = signal(
        space
            .as_ref()
            .map(|s| s.avatar.clone())
            // The default space avatar is always exists in DB. It is checked during the app start up.
            .unwrap_or_else(|| {
                Avatar::new(
                    common::DEFAULT_SPACE_AVATAR_ID.into(),
                    common::DEFAULT_SPACE_AVATAR_PATH,
                )
            }),
    );
    let ref_input = NodeRef::<html::Input>::new();

    Effect::new(move |_| {
        ref_input.on_load(|input| {
            if let Err(err) = input.focus() {
                warn!(?err, "Can not focus TextArea");
            }
        });
    });

    let gen_avatar_toaster = toaster.clone();
    let generate_avatar = Callback::new(move |_| {
        let toaster = gen_avatar_toaster.clone();
        spawn_local(async move {
            set_avatar.set(try_exec!(gen_avatar().await, "Failed to generate a new avatar:", toaster).into());
        });
    });

    let pick_avatar = Callback::new(move |_| {
        let toaster = toaster.clone();
        spawn_local(async move {
            if let Some(avatar_image_file) = try_exec!(pick_avatar().await, "Failed to generate a new avatar:", toaster)
            {
                set_avatar.set(avatar_image_file.into());
            }
        });
    });

    let id = space.as_ref().map(|s| s.id);
    let create_space = move || {
        let name = space_name.get();
        let avatar = avatar.get();
        on_cancel.run(());

        let action = async move {
            if let Some(id) = id {
                update_space(UpdateSpace {
                    id,
                    name: name.into(),
                    avatar,
                })
                .await
                .expect("Space updating should not fail");

                None
            } else {
                let new_space_id = Uuid::new_v4();
                create_space(CreateSpace {
                    id: new_space_id.into(),
                    name: name.into(),
                    avatar,
                })
                .await
                .expect("Space creation should not fail");

                Some(new_space_id)
            }
        };

        spawn_local(async move {
            let new_space_id = action.await;
            let spaces = list_spaces().await.expect("list spaces should not fail");
            let new_current_space = new_space_id
                .and_then(|new_space_id| spaces.iter().find(|space| *space.id.as_ref() == new_space_id).cloned());

            set_spaces.set(spaces);
            if let Some(space) = new_current_space {
                set_selected_space.run((space,));
            }
        });
    };

    let regenerate_space_avatar = config.key_bindings.regenerate_space_avatar.clone();
    let space_form_element = NodeRef::new();

    let _ = use_event_listener(space_form_element, keydown, move |ev| {
        let key = ev.key();

        if key == "Escape" {
            ev.prevent_default();
            on_cancel.run(());
        } else if key == "Enter" {
            ev.prevent_default();
            create_space();
        } else if regenerate_space_avatar.matches(&ev) {
            ev.prevent_default();
            generate_avatar.run(());
        }
    });

    let global_config = expect_context::<RwSignal<Config>>();

    view! {
        <div class="create-space-window" node_ref=space_form_element>
            {if space.is_some() {
                view! { <span class="create-space-title">"Update space"</span> }
            } else {
                view! { <span class="create-space-title">"Create space"</span> }
            }}
            <div class="create-space-avatar">
                <img class="create-space-avatar-img" src=move || convert_file_src(avatar.get().path(), &global_config.get().app.base_path) />
                <div style="display: inline-flex; gap: 5px; justify-content: center;">
                    <button class="tool" title="Regenerate avatar" on:click=move |_| generate_avatar.run(())>
                        <img alt="regenerate-avatar" src="/public/icons/refresh.svg" />
                    </button>
                    <button class="tool" title="Pick custom image" on:click=move |_| pick_avatar.run(())>
                        <img alt="pick-custom-image" src="/public/icons/camera-light.png" />
                    </button>
                </div>
            </div>
            <input
                id=INPUT_ELEM_ID
                node_ref=ref_input
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
                    on:click=move |_| on_cancel.run(())
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
