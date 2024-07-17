use common::space::{DeleteSpace, OwnedSpace};
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use crate::backend::spaces::{delete_space, list_spaces};
use crate::common::{Confirm, Modal};
use crate::spaces::space_form::SpaceForm;

#[component]
pub fn Info(current_space: OwnedSpace, set_spaces: SignalSetter<Vec<OwnedSpace>>) -> impl IntoView {
    let config = expect_context::<RwSignal<Config>>();
    let (key_bindings, _) = create_slice(config, |config| config.key_bindings.clone(), |_config, _: ()| {});

    let (show_edit_modal, set_show_edit_modal) = create_signal(false);
    let (show_delete_modal, set_show_delete_modal) = create_signal(false);

    let delete_space = move || {
        let id = current_space.id;
        spawn_local(async move {
            delete_space(DeleteSpace { id })
                .await
                .expect("space deleting should not fail");
            set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
        });
    };

    let current_space_name = current_space.name.to_string();
    let space = Some(current_space.clone());

    view! {
        <div class="info">
            {move || {
                let key_bindings = key_bindings.get();

                use_hotkeys!((key_bindings.edit_current_space) => move |_| {
                    set_show_edit_modal.set(true);
                });

                use_hotkeys!((key_bindings.delete_current_space) => move |_| {
                    set_show_delete_modal.set(true);
                });

                view! {}
            }}
            <span class="space-name">{current_space_name.clone()}</span>
            <div>
                <div class="horizontal">
                    <button
                        class="tool"
                        title="Edit space info"
                        on:click=move |_| set_show_edit_modal.set(true)
                    >
                        <img alt="change space name" src="/public/icons/edit-space.svg" />
                    </button>
                    <button
                        class="tool"
                        title="Delete space"
                        on:click=move |_| set_show_delete_modal.set(true)
                    >
                        <img alt="delete space" src="/public/icons/delete-space.png" />
                    </button>
                </div>
            </div>
            <Show when=move || show_delete_modal.get()>
                <Confirm
                    message=format!("Confirm '{}' space deletion.", current_space.name.as_ref())
                    on_confirm=move |_| delete_space()
                    on_cancel=move |_| set_show_delete_modal.set(false)
                />
            </Show>
            <Show when=move || show_edit_modal.get()>{
                let space = space.clone();
                view! {
                    <Modal>
                        <SpaceForm
                            space
                            on_cancel=move |_| set_show_edit_modal.set(false)
                            set_spaces
                        />
                    </Modal>
                }
            }</Show>
        </div>
    }
}
