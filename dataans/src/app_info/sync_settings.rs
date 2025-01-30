use common::profile::{Sync, SyncMode, UserContext};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::notes::md_node::InlineCode;

#[component]
pub fn SyncSettings(context: UserContext) -> impl IntoView {
    let user_context = expect_context::<RwSignal<Option<UserContext>>>();

    let UserContext {
        user_id,
        username,
        sync_config,
    } = context;

    let name = username.clone();
    let url = sync_config.get_web_server_url();
    let toggle_sync_availability = Callback::new(move |sync_enabled| {
        if sync_enabled {
            user_context.set(Some(UserContext {
                user_id,
                username: name.clone(),
                sync_config: Sync::Enabled {
                    url: url.clone(),
                    mode: SyncMode::Manual,
                },
            }));
        } else {
            user_context.set(Some(UserContext {
                user_id,
                username: name.clone(),
                sync_config: Sync::Disabled { url: url.clone() },
            }));
        }
    });

    view! {
        <div class="app-info-sync-config">
            <div class="horizontal">
                <span>
                    "Signed in as "
                    <InlineCode code={username.as_ref().to_string()} />
                    " (id: "
                    <InlineCode code={user_id.as_ref().to_string()} />
                    "). "
                </span>
                {if sync_config.is_enabled() { view! {
                    <button
                        class="button_ok"
                        on:click=move |_| toggle_sync_availability.call(false)
                    >
                        "Disable sync"
                    </button>
                }} else { view! {
                    <button
                        class="button_ok"
                        on:click=move |_| toggle_sync_availability.call(true)
                    >
                        "Enable sync"
                    </button>
                }}}
            </div>
            {if let Some(mode) = sync_config.mode() {
                let name = username.clone();
                let url = sync_config.get_web_server_url();
                let set_sync_mode = move |mode| {
                    user_context.set(Some(UserContext {
                        user_id,
                        username: name.clone(),
                        sync_config: Sync::Enabled {
                            url: url.clone(),
                            mode,
                        }
                    }));
                };
                let on_change = Callback::new(move |ev: leptos::ev::Event| {
                    let mode: HtmlInputElement = ev.target().unwrap().unchecked_into();
                    let value = mode.value();
                    if value == "manual" {
                        set_sync_mode(SyncMode::Manual);
                    }
                    if value == "poll" {
                        set_sync_mode(SyncMode::Poll {
                            period: time::Duration::hours(2),
                        });
                    }
                    if value == "push" {
                        set_sync_mode(SyncMode::Push);
                    }
                });

                view! {
                    <form>
                        <div>
                            <input type="radio" id="manual" name="sync-move" value="manual" on:change=move |ev| on_change.call(ev) checked=mode == SyncMode::Manual />
                            <label for="manual">"Manual"</label>
                        </div>
                        <div>
                            <input type="radio" id="poll" name="sync-move" value="poll" on:change=move |ev| on_change.call(ev) checked=matches!(mode, SyncMode::Poll { .. }) />
                            <label for="poll">"Poll"</label>
                        </div>
                        <div>
                            <input type="radio" id="push" name="sync-move" value="push" on:change=move |ev| on_change.call(ev) checked=mode == SyncMode::Push />
                            <label for="push">"Push"</label>
                        </div>
                    </form>
                }
            } else { view! {
                <form />
            }}}
        </div>
    }
}

#[component]
pub fn SetUpSync() -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let web_server_url_ref: NodeRef<html::Input> = NodeRef::new();
    let show_auth_window = Callback::new(move |_: ()| {
        let t = toaster.clone();
        let url = web_server_url_ref.get().expect("<input> should be mounted").value();
        let url = try_exec!(url.parse(), "Failed to parse the web server URL", t);
        spawn_local(async move {
            try_exec!(
                crate::backend::window::show_auth_window(url).await,
                "Failed to create auth window",
                t
            );
        })
    });

    view! {
        <div class="horizontal">
            <input type="text" class="input" value="http://127.0.0.1:8000/" style="flex-grow: 1;" node_ref=web_server_url_ref />
            <button on:click=move |_| show_auth_window.call(()) title="Set up back up & sync" class="tool">
                <img alt="cloud-icon" src="/public/icons/cloud-backup-light.png" />
            </button>
        </div>
    }
}

#[component]
pub fn SyncState() -> impl IntoView {
    let user_context = expect_context::<RwSignal<Option<UserContext>>>();

    move || {
        if let Some(user_context) = user_context.get() {
            view! {
                <SyncSettings context=user_context />
            }
        } else {
            view! {
                <SetUpSync />
            }
        }
    }
}
