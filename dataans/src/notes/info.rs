use common::Config;
use common::space::{DeleteSpace, Id as SpaceId, OwnedSpace};
use leptos::callback::Callback;
use leptos::ev::keydown;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_use::{use_document, use_event_listener};

use crate::backend::spaces::delete_space;
use crate::common::{Confirm, Modal};
use crate::dom::MatchKeyBinding;
use crate::spaces::space_form::SpaceForm;

#[component]
pub fn Info(
    current_space: OwnedSpace,
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
    delete_state_space: SignalSetter<SpaceId>,
    #[prop(into)] toggle_note_search: Callback<(), ()>,
    #[prop(into)] set_selected_space: Callback<(OwnedSpace,), ()>,
    config: Config,
) -> impl IntoView {
    let (show_edit_modal, set_show_edit_modal) = signal(false);
    let (show_delete_modal, set_show_delete_modal) = signal(false);

    let delete_space = move || {
        let id = current_space.id;
        spawn_local(async move {
            delete_space(DeleteSpace { id })
                .await
                .expect("space deleting should not fail");
            delete_state_space.set(id);
        });
    };

    let current_space_name = current_space.name.to_string();

    let key_bindings = &config.key_bindings;
    let edit_current_space = key_bindings.edit_current_space.clone();
    let delete_current_space = key_bindings.delete_current_space.clone();
    let find_note_in_selected_space = key_bindings.find_note_in_selected_space.clone();

    let _ = use_event_listener(use_document(), keydown, move |ev| {
        if edit_current_space.matches(&ev) {
            ev.prevent_default();
            set_show_edit_modal.set(true);
        }

        if delete_current_space.matches(&ev) {
            ev.prevent_default();
            set_show_delete_modal.set(true);
        }

        if find_note_in_selected_space.matches(&ev) {
            ev.prevent_default();
            toggle_note_search.run(());
        }
    });

    let space = Some(current_space.clone());

    view! {
        <div class="info">
            <span class="space-name">{current_space_name.clone()}</span>
            <div>
                <div class="horizontal">
                    <button
                        class="tool"
                        title="Find note"
                        on:click=move |_| toggle_note_search.run(())
                    >
                        <img alt="find note" src="/public/icons/search.svg" />
                    </button>
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
                    on_confirm=move || delete_space()
                    on_cancel=move || set_show_delete_modal.set(false)
                />
            </Show>
            <Show when=move || show_edit_modal.get()>{
                let space = space.clone();
                let config = config.clone();
                view! {
                    <Modal>
                        <SpaceForm
                            space
                            on_cancel=move || set_show_edit_modal.set(false)
                            set_spaces
                            set_selected_space
                            config
                        />
                    </Modal>
                }
            }</Show>
        </div>
    }
}
