use common::space::OwnedSpace;
use leptos::*;

use crate::common::Modal;
use crate::spaces::space_form::SpaceForm;

#[component]
pub fn Tools(set_spaces: SignalSetter<Vec<OwnedSpace>>) -> impl IntoView {
    let (show_modal, set_show_modal) = create_signal(false);

    view! {
        <div class="tools">
            <button class="tool" title="Add a new space" on:click=move |_| set_show_modal.set(true)>
                <img alt="add-space" src="/public/icons/add-space-1.png" />
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
