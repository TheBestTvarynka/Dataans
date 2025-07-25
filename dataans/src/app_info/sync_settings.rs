use common::profile::{Sync, SyncMode, UserContext};
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;

#[component]
pub fn SyncSettings(context: UserContext) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let sign_out = Callback::new(move |_: ()| {
        let t = toaster.clone();
        spawn_local(async move {
            try_exec!(crate::backend::auth::sign_out().await, "Failed to sign out", t);
        })
    });

    let UserContext {
        sync_config: Sync { url, mode },
    } = context;
    let url = url.as_ref().to_string();

    view! {
        <div class="app-info-sync-config">
            <div class="horizontal">
                <input type="text" class="input" value=url style="flex-grow: 1;" disabled=true />
                <button title="Sign out" class="tool" on:click=move |_| sign_out.call(())>
                    <img alt="cloud-icon" src="/public/icons/sign-out.png" />
                </button>
            </div>
            {match mode {
                SyncMode::Manual => view! {
                    <form>
                        <div class="horizontal">
                            <input type="radio" id="manual" name="sync-move" value="manual" checked=mode == SyncMode::Manual />
                            <label for="manual" class="app-info-sync-mode">
                                <b>"Manual."</b>
                                <span>"You control when the sync happens. The data is being synchronized only when you press the button to sync it."</span>
                            </label>
                        </div>
                    </form>
                }
            }}
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
                crate::backend::window::show_cf_auth_window(&url).await,
                "Failed to create auth window",
                t
            );
        })
    });

    view! {
        <div class="horizontal">
            <input type="text" class="input" value="https://backup.dataans.com/" style="flex-grow: 1;" node_ref=web_server_url_ref />
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
