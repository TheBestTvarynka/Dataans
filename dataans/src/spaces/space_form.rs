use common::space::{Avatar, CreateSpace, OwnedSpace, UpdateSpace};
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};
use uuid::Uuid;

use crate::backend::convert_file_src;
use crate::backend::file::gen_avatar;
use crate::backend::spaces::{create_space, list_spaces, update_space};

const INPUT_ELEM_ID: &str = "space-form";

#[component]
pub fn SpaceForm(
    space: Option<OwnedSpace>,
    #[prop(into)] on_cancel: Callback<(), ()>,
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
    set_selected_space: Callback<OwnedSpace, ()>,
    config: Config,
) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (space_name, set_space_name) = create_signal(space.as_ref().map(|s| s.name.to_string()).unwrap_or_default());
    let (avatar, set_avatar) = create_signal(
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
    let ref_input = create_node_ref::<html::Input>();

    create_effect(move |_| {
        if let Some(ref_input) = ref_input.get() {
            let _ = ref_input.on_mount(|input| {
                if let Err(err) = input.focus() {
                    warn!("Can not focus TextArea: {:?}", err);
                }
            });
        }
    });

    let generate_avatar = Callback::new(move |_| {
        let toaster = toaster.clone();
        spawn_local(async move {
            set_avatar.set(try_exec!(gen_avatar().await, "Failed to generate a new avatar:", toaster).into());
        });
    });

    let id = space.as_ref().map(|s| s.id);
    let create_space = move || {
        let name = space_name.get();
        let avatar = avatar.get();
        on_cancel.call(());

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
                set_selected_space.call(space);
            }
        });
    };

    use_hotkeys!(("Escape") => move |_| on_cancel.call(()));
    use_hotkeys!(("Enter") => move |_| create_space());
    let regenerate_space_avatar = config.key_bindings.regenerate_space_avatar.clone();
    use_hotkeys!((regenerate_space_avatar) => move |_| generate_avatar.call(()));

    let global_config = expect_context::<RwSignal<Config>>();

    view! {
        <div class="create-space-window" on:load=move |_| info!("on_load")>
            {if space.is_some() {
                view! { <span class="create-space-title">"Update space"</span> }
            } else {
                view! { <span class="create-space-title">"Create space"</span> }
            }}
            <div class="create-space-avatar">
                <img class="create-space-avatar-img" src=move || convert_file_src(avatar.get().path(), &global_config.get().app.base_path) />
                <div style="align-self: center">
                    <button class="tool" title="Regenerate avatar" on:click=move |_| generate_avatar.call(())>
                        <img alt="regenerate-avatar" src="/public/icons/refresh.svg" />
                    </button>
                </div>
            </div>
            <input
                id=INPUT_ELEM_ID
                _ref=ref_input
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
                    on:click=move |_| create_space()
                >
                    {if space.is_some() { "Update" } else { "Create" }}
                </button>
            </div>
        </div>
    }
}
