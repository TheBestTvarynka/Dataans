use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};
use web_sys::KeyboardEvent;

use crate::common::Modal;
use crate::spaces::space_form::SpaceForm;
use crate::utils::focus_element;
use crate::FindNoteMode;

pub const SEARCH_NOTE_INPUT_ID: &str = "search-note-input";

#[component]
pub fn Tools(
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
    spaces_minimized: Signal<bool>,
    set_spaces_minimized: SignalSetter<bool>,
    set_find_node_mode: SignalSetter<FindNoteMode>,
    set_query: SignalSetter<String>,
    config: Config,
) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    let class = move || {
        if spaces_minimized.get() {
            "tools tools-vertical"
        } else {
            "tools"
        }
    };

    let key_bindings = &config.key_bindings;

    use_hotkeys!((key_bindings.create_space.clone()) => move |_| {
        set_show_modal.set(true);
    });

    use_hotkeys!((key_bindings.find_note.clone()) => move |_| {
        if spaces_minimized.get() {
            set_spaces_minimized.set(false);
        }
        set_find_node_mode.set(FindNoteMode::FindNote { space: None });
        focus_element(SEARCH_NOTE_INPUT_ID);
    });

    let key_down = move |key: KeyboardEvent| {
        if key.key() == "Enter" {
            key.prevent_default();
            set_find_node_mode.set(FindNoteMode::FindNote { space: None });
        }
    };

    view! {
        <div class=class>
            <Show when=move || !spaces_minimized.get()>
                <button class="tool" title="Add a new space" on:click=move |_| set_show_modal.set(true)>
                    <img alt="add-space" src="/public/icons/add-space-1.png" />
                </button>
            </Show>
            <input
                id=SEARCH_NOTE_INPUT_ID
                type="text"
                placeholder="Search note..."
                class="input"
                style=move || if spaces_minimized.get() {
                    "display: none; flex-grow: 1"
                } else {
                    "flex-grow: 1"
                }
                on:input=move |ev| set_query.set(event_target_value(&ev))
                on:keydown=key_down
                // prop:value=space_name
            />
            <button class="tool" title="Toggle panel" on:click=move |_| set_spaces_minimized.set(!spaces_minimized.get())>
                {move || if spaces_minimized.get() {
                    view! {
                        <img alt="maximize-spaces" src="/public/icons/side-right.png" />
                    }
                } else {
                    view! {
                        <img alt="minimize-spaces" src="/public/icons/side-left.png" />
                    }
                }}
            </button>
            <Show when=move || show_modal.get()>
                {
                    let config = config.clone();
                    view! {
                        <Modal>
                            <SpaceForm
                                space=None
                                on_cancel=move |_| set_show_modal.set(false)
                                set_spaces
                                config
                            />
                        </Modal>
                    }
                }
            </Show>
        </div>
    }
}
