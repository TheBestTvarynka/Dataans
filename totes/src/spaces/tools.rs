use common::space::OwnedSpace;
use common::Config;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use crate::common::Modal;
use crate::spaces::space_form::SpaceForm;

#[component]
pub fn Tools(
    set_spaces: SignalSetter<Vec<OwnedSpace>>,
    spaces_minimized: Signal<bool>,
    set_spaces_minimized: SignalSetter<bool>,
) -> impl IntoView {
    let config = expect_context::<RwSignal<Config>>();
    let (key_bindings, _) = create_slice(config, |config| config.key_bindings.clone(), |_config, _: ()| {});

    let (show_modal, set_show_modal) = create_signal(false);

    let class = move || {
        if spaces_minimized.get() {
            "tools tools-vertical"
        } else {
            "tools"
        }
    };

    view! {
        <div class={class}>
            {move || {
                let key_bindings = key_bindings.get();

                use_hotkeys!((key_bindings.create_space) => move |_| {
                    set_show_modal.set(true);
                });

                view! {
                    <div style="display: none" />
                }
            }}
            <button class="tool" title="Add a new space" on:click=move |_| set_show_modal.set(true)>
                <img alt="add-space" src="/public/icons/add-space-1.png" />
            </button>
            <button class="tool" title="Minimize spaces panel" on:click=move |_| set_spaces_minimized.set(!spaces_minimized.get())>
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
                <Modal>
                    <SpaceForm
                        space=None
                        on_cancel=move |_| set_show_modal.set(false)
                        set_spaces
                    />
                </Modal>
            </Show>
        </div>
    }
}
