mod export;
mod import;

use std::path::PathBuf;

use common::{App, Appearance, Config, KeyBindings};
use leptos::*;

use crate::app_info::export::Export;
use crate::backend::{open_config_file, open_config_file_folder, open_theme_file};

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

    let show_auth_window = move |_| {
        let t = toaster.clone();
        spawn_local(async move {
            try_exec!(
                crate::backend::window::show_auth_window().await,
                "Failed to create auth window",
                t
            );
        })
    };

    view! {
        <div class="app-into-window">
            <span class="app-into-window-title">{format!("Dataans v.{}", env!("CARGO_PKG_VERSION"))}</span>
            <span>"Take notes in the form of markdown snippets grouped into spaces."</span>
            <span>"Source code: "<a href="https://github.com/TheBestTvarynka/Dataans" target="_blank">"GitHub/TbeBestTvarynka/Dataans"</a>"."</span>
            <span class="icons-by-icons8">"Icons by "<a href="https://icons8.com" target="_blank">"Icons8"</a>"."</span>
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
                <button on:click=show_auth_window title="Set up back up & sync" class="tool">
                    <img alt="cloud-icon" src="/public/icons/cloud-backup-light.png" />
                </button>
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
                                <span class="inline-code">{app_toggle}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Always on top"</td>
                            <td>
                                <span class="inline-code">{always_on_top}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Hide window decorations"</td>
                            <td>
                                <span class="inline-code">{hide_window_decorations}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Hide app taskbar icon"</td>
                            <td>
                                <span class="inline-code">{hide_taskbar_icon}</span>
                            </td>
                        </tr>

                        // Appearance config
                        <tr>
                            <th colspan="2">"Appearance"</th>
                        </tr>
                        <tr>
                            <td>"Theme file"</td>
                            <td class="horizontal">
                                <span class="inline-code">{theme.display().to_string()}</span>
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
                                <span class="inline-code">{toggle_spaces_bar}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Create a new space"</td>
                            <td>
                                <span class="inline-code">{create_space}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Edit current space"</td>
                            <td>
                                <span class="inline-code">{edit_current_space}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Delete current space"</td>
                            <td>
                                <span class="inline-code">{delete_current_space}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Select next item in the side bar list"</td>
                            <td>
                                <span class="inline-code">{select_next_list_item}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Select previous item in the side bar list"</td>
                            <td>
                                <span class="inline-code">{select_prev_list_item}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Search notes globally"</td>
                            <td>
                                <span class="inline-code">{find_note}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Search notes in the current space"</td>
                            <td>
                                <span class="inline-code">{find_note_in_selected_space}</span>
                            </td>
                        </tr>
                        <tr>
                            <td>"Regenerate space avatar image"</td>
                            <td>
                                <span class="inline-code">{regenerate_space_avatar}</span>
                            </td>
                        </tr>
                    </table>
                }
            }}
            <hr style="width: 80%" />
            <Export />
        </div>
    }
}
