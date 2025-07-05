use common::profile::{Sync, SyncMode, UserContext};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn SyncSettings(context: UserContext) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let UserContext { sync_config } = context;

    let url = sync_config.get_web_server_url();
    let t = toaster.clone();
    let toggle_sync_availability = Callback::new(move |sync_enabled| {
        let sync_config = if sync_enabled {
            Sync::Enabled {
                url: url.clone(),
                mode: SyncMode::Manual,
            }
        } else {
            Sync::Disabled { url: url.clone() }
        };
        let t = t.clone();
        spawn_local(async move {
            try_exec!(
                crate::backend::sync::set_sync_options(&sync_config).await,
                "Failed to set sync options",
                t
            );
        });
    });

    view! {
        <div class="app-info-sync-config">
            <div class="horizontal">
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
                let url = sync_config.get_web_server_url();
                let t = toaster.clone();
                let on_change = Callback::new(move |ev: leptos::ev::Event| {
                    let mode: HtmlInputElement = ev.target().unwrap().unchecked_into();
                    let mode = match mode.value().as_str() {
                        "manual" => SyncMode::Manual,
                        "push" => SyncMode::Push,
                        _ => unreachable!(),
                    };

                    let sync_config = Sync::Enabled {
                        url: url.clone(),
                        mode,
                    };
                    let t = t.clone();
                    spawn_local(async move {
                        try_exec!(
                            crate::backend::sync::set_sync_options(&sync_config).await,
                            "Failed to set sync options",
                            t
                        );
                    });
                });

                view! {
                    <form>
                        <div class="horizontal">
                            <input type="radio" id="manual" name="sync-move" value="manual" on:change=move |ev| on_change.call(ev) checked=mode == SyncMode::Manual />
                            <label for="manual" class="app-info-sync-mode">
                                <b>"Manual."</b>
                                <span>"You control when the sync happens. The data is being synchronized only when you press the button to sync it."</span>
                            </label>
                        </div>
                        <div class="horizontal">
                            <input type="radio" id="push" name="sync-move" value="push" on:change=move |ev| on_change.call(ev) checked=mode == SyncMode::Push disabled=true />
                            <label for="push" class="app-info-sync-mode">
                                <b>"Push."</b>
                                <span>"The app maintains the connection with a server and syncs data automatically. (not implemented yet)"</span>
                            </label>
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
