mod export;
mod import;
mod sync_settings;

use std::path::PathBuf;

use common::{App, Appearance, Config, KeyBindings};
use leptos::prelude::*;
use leptos::task::spawn_local;

use self::sync_settings::SyncState;
use crate::app_info::export::Export;
use crate::app_info::import::Import;
use crate::backend::{open_config_file, open_config_file_folder, open_theme_file};
use crate::notes::md_node::InlineCode;

#[component]
pub fn AppInfo() -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let global_config = expect_context::<RwSignal<Config>>();

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

    view! {
        <div class="app-into-window">
            <span>"Take notes in the form of markdown snippets grouped into spaces."</span>
            <span>"Source code: "<a href="https://github.com/TheBestTvarynka/Dataans" target="_blank">"GitHub/TbeBestTvarynka/Dataans"</a>"."</span>
            <span class="icons-by-icons8">"Icons by "<a href="https://icons8.com" target="_blank">"Icons8"</a>"."</span>
            <hr style="width: 100%" />
            <SyncState />
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
                let App { app_toggle, always_on_top, hide_window_decorations, hide_taskbar_icon, base_path } = app;

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
                            <td>"App data folder"</td>
                            <td>
                                <InlineCode code=base_path />
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
                                    title="Edit theme file"
                                    on:click=move |_| open_theme_file(theme.clone())
                                >
                                    <img alt="edit note" src="/public/icons/edit-space.svg" />
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
            <div class="app-info-window-data-actions">
                <Export />
                <Import />
            </div>
        </div>
    }
}
