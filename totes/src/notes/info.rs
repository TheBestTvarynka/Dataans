use common::space::{Space, UpdateSpace};
use leptos::*;

use crate::backend::spaces::{list_spaces, update_space};

#[component]
pub fn Info(current_space: Space<'static>, set_spaces: SignalSetter<Vec<Space<'static>>>) -> impl IntoView {
    let (show_input, set_show_input) = create_signal(false);
    let (space_name, set_space_name) = create_signal(current_space.name.to_string());
    info!("{}", current_space.name.to_string());

    let update_space = move || {
        let id = current_space.id;
        let name = space_name.get();
        spawn_local(async move {
            update_space(UpdateSpace { id, name: name.into() })
                .await
                .expect("space updating should not fail");
            set_spaces.set(list_spaces().await.expect("list spaces should not fail"));
            set_show_input.set(false);
        });
    };

    let key_down = move |key| {
        if key == "Enter" {
            update_space();
        } else if key == "Escape" {
            set_show_input.set(false);
        }
    };

    view! {
        <div class="info">
            {move || match show_input.get() {
                true => view! {
                    <input
                        type="text"
                        placeholder="Space name"
                        class="input"
                        on:input=move |ev| set_space_name.set(event_target_value(&ev))
                        on:keydown=move |ev| key_down(ev.key())
                        prop.value=space_name
                        value=space_name
                    />
                }.into_any(),
                false => view! {
                    <span class="space-name">{String::from(current_space.name.clone())}</span>
                }.into_any(),
            }}
            <div>
                {move || match show_input.get() {
                    true => view! {
                        <div class="horizontal">
                            <button
                                class="button_ok"
                                on:click=move |_| update_space()
                            >
                                "Ok"
                            </button>
                            <button
                                class="button_cancel"
                                on:click=move |_| set_show_input.set(false)
                            >
                                "Cancel"
                            </button>
                        </div>
                    }.into_any(),
                    false => view! {
                        <div class="horizontal">
                            <button
                                class="tool"
                                title="Change space name"
                                on:click=move |_| set_show_input.set(true)
                            >
                                <img alt="change space name" src="/public/icons/edit-space.svg" />
                            </button>
                            <button class="tool" title="Delete space">
                                <img alt="delete space" src="/public/icons/delete-space.png" />
                            </button>
                        </div>
                    }.into_any(),
                }}
            </div>
        </div>
    }
}
