mod export;
mod sync_settings;

use std::path::PathBuf;

use common::profile::{Sync, SyncMode, UserContext};
use common::{App, Appearance, Config, KeyBindings};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::app_info::export::Export;
use crate::backend::{open_config_file, open_config_file_folder, open_theme_file};
use crate::notes::md_node::InlineCode;

#[component]
pub fn AppInfo() -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let global_config = expect_context::<RwSignal<Config>>();
    let user_context = expect_context::<RwSignal<Option<UserContext>>>();

    let open_config_file = move |_| spawn_local(open_config_file());
    let open_config_file_folder = move |_| spawn_local(open_config_file_folder());
    let open_theme_file = move |theme: PathBuf| spawn_local(async move { open_theme_file(&theme).await });

    let (is_autostart_enabled, set_autostart) = create_signal(false);

    let t = toaster.clone();
    let enable_autostart = Callback::new(move |_| {
        let t = t.clone();
        spawn_local(async move {
            set_autostart.set(try_exec!(
                crate::backend::autostart::enable().await,
                "Failed to enable autostart",
                t
            ));
        })
    });

    let t = toaster.clone();
    let disable_autostart = Callback::new(move |_| {
        let t = t.clone();
        spawn_local(async move {
            set_autostart.set(try_exec!(
                crate::backend::autostart::disable().await,
                "Failed to disable autostart",
                t
            ));
        })
    });

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
        <div class="app-into-window">
            <span>"Take notes in the form of markdown snippets grouped into spaces."</span>
            <span>"Source code: "<a href="https://github.com/TheBestTvarynka/Dataans" target="_blank">"GitHub/TbeBestTvarynka/Dataans"</a>"."</span>
            <span class="icons-by-icons8">"Icons by "<a href="https://icons8.com" target="_blank">"Icons8"</a>"."</span>
            <hr style="width: 100%" />
                {move || if let Some(context) = user_context.get() {
                    let UserContext { user_id, username, sync_config } = context;

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
                                sync_config: Sync::Disabled { url: url.clone(), },
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
                } else { view! {
                    <div class="horizontal">
                        <input type="text" class="input" value="http://127.0.0.1:8000/" style="flex-grow: 1;" node_ref=web_server_url_ref />
                        <button on:click=move |_| show_auth_window.call(()) title="Set up back up & sync" class="tool">
                            <img alt="cloud-icon" src="/public/icons/cloud-backup-light.png" />
                        </button>
                    </div>
                }} }
            <hr style="width: 80%" />
            <div class="horizontal">
                <button class="button_ok" on:click=open_config_file>"Edit config file"</button>
                <button
                    class="tool"
                    title="Open config file location"
                    on:click=open_config_file_folder
                >
                    <img alt="edit note" src="/public/icons/folder-light.png" />
                </button>
                {move || if is_autostart_enabled.get() {view! {
                    <button class="button_ok" on:click=move |ev| disable_autostart.call(ev) title="Disable autostart">"Disable autostart"</button>
                }} else {view! {
                    <button class="button_ok" on:click=move |ev| enable_autostart.call(ev)  title="Enable autostart">"Enable autostart"</button>
                }}}
            </div>
            {move || {
                let Config { key_bindings, appearance, app } = global_config.get();

                let KeyBindings { toggle_spaces_bar, create_space, edit_current_space, delete_current_space, select_next_list_item, select_prev_list_item, find_note, find_note_in_selected_space, regenerate_space_avatar } = key_bindings;
                let Appearance { theme } = appearance;
                let App { app_toggle, always_on_top, hide_window_decorations, hide_taskbar_icon } = app;

                view!{
                    <table class="app-window-config-table">
                        // App config
                        <tr>
                            <th colspan="2">"App"</th>
                        </tr>
                        <tr>
                            <td>"App window toggle"</td>
                            <td>
                                <InlineCode code=app_toggle />
                            </td>
                        </tr>
                        <tr>
                            <td>"Always on top"</td>
                            <td>
                                <InlineCode code=always_on_top.to_string() />
                            </td>
                        </tr>
                        <tr>
                            <td>"Hide window decorations"</td>
                            <td>
                                <InlineCode code=hide_window_decorations.to_string() />
                            </td>
                        </tr>
                        <tr>
                            <td>"Hide app taskbar icon"</td>
                            <td>
                                <InlineCode code=hide_taskbar_icon.to_string() />
                            </td>
                        </tr>

                        // Appearance config
                        <tr>
                            <th colspan="2">"Appearance"</th>
                        </tr>
                        <tr>
                            <td>"Theme file"</td>
                            <td class="horizontal">
                                <InlineCode code=theme.display().to_string() />
                                <button
                                    class="tool"
                                    title="Open theme file location"
                                    on:click=move |_| open_theme_file(theme.clone())
                                >
                                    <img alt="edit note" src="/public/icons/folder-light.png" />
                                </button>
                            </td>
                        </tr>

                        // Keyboard shortcuts config
                        <tr>
                            <th colspan="2">"Keyboard Shortcuts"</th>
                        </tr>
                        <tr>
                            <td>"Toggle side bar"</td>
                            <td>
                                <InlineCode code=toggle_spaces_bar />
                            </td>
                        </tr>
                        <tr>
                            <td>"Create a new space"</td>
                            <td>
                                <InlineCode code=create_space />
                            </td>
                        </tr>
                        <tr>
                            <td>"Edit current space"</td>
                            <td>
                                <InlineCode code=edit_current_space />
                            </td>
                        </tr>
                        <tr>
                            <td>"Delete current space"</td>
                            <td>
                                <InlineCode code=delete_current_space />
                            </td>
                        </tr>
                        <tr>
                            <td>"Select next item in the side bar list"</td>
                            <td>
                                <InlineCode code=select_next_list_item />
                            </td>
                        </tr>
                        <tr>
                            <td>"Select previous item in the side bar list"</td>
                            <td>
                                <InlineCode code=select_prev_list_item />
                            </td>
                        </tr>
                        <tr>
                            <td>"Search notes globally"</td>
                            <td>
                                <InlineCode code=find_note />
                            </td>
                        </tr>
                        <tr>
                            <td>"Search notes in the current space"</td>
                            <td>
                                <InlineCode code=find_note_in_selected_space />
                            </td>
                        </tr>
                        <tr>
                            <td>"Regenerate space avatar image"</td>
                            <td>
                                <InlineCode code=regenerate_space_avatar />
                            </td>
                        </tr>
                    </table>
                }
            }}
            <hr style="width: 100%" />
            <Export />
        </div>
    }
}
