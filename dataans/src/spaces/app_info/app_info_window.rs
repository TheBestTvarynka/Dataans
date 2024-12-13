use std::path::PathBuf;

use common::{App, Appearance, Config, DataExportConfig, ExportFormat, KeyBindings, NotesExportOption};
use leptos::*;
use leptos_hotkeys::use_hotkeys;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::backend::export::export_data;
use crate::backend::file::open;
use crate::backend::{open_config_file, open_config_file_folder, open_theme_file};

#[component]
pub fn AppInfoWindow(#[prop(into)] close: Callback<(), ()>) -> impl IntoView {
    use_hotkeys!(("Escape") => move |_| close.call(()));

    let global_config = expect_context::<RwSignal<Config>>();

    let open_config_file = move |_| spawn_local(open_config_file());
    let open_config_file_folder = move |_| spawn_local(open_config_file_folder());
    let open_theme_file = move |theme: PathBuf| spawn_local(async move { open_theme_file(&theme).await });

    let (is_autostart_enabled, set_autostart) = create_signal(false);

    let enable_autostart = move |_| {
        spawn_local(async move {
            let flag = crate::backend::autostart::enable().await;
            set_autostart.set(flag);
        })
    };

    let disable_autostart = move |_| {
        spawn_local(async move {
            let flag = crate::backend::autostart::disable().await;
            set_autostart.set(flag);
        })
    };

    let (export_config, set_export_config) = create_signal(None);
    let (export_option, set_export_option) = create_signal(NotesExportOption::OneFile);
    let (export_format, set_export_format) = create_signal(ExportFormat::Md);

    let export_result = create_resource(
        move || export_config.get(),
        move |export_config| async move {
            if let Some(export_config) = export_config {
                Some(export_data(export_config).await)
            } else {
                None
            }
        },
    );
    let export_data = move |_| {
        set_export_config.set(Some(DataExportConfig {
            notes_export_option: export_option.get(),
            format: export_format.get(),
        }));
    };
    let open_exported_data_dir = move |path: PathBuf| spawn_local(async move { open(&path).await });

    view! {
        <div class="app-into-window">
            <button
                class="tool app-window-close-button"
                title="Close window"
                on:click=move |_| close.call(())
            >
                <img alt="edit note" src="/public/icons/cancel.png" />
            </button>
            <span class="app-into-window-title">{format!("Dataans v.{}", env!("CARGO_PKG_VERSION"))}</span>
            <span>"Take notes in the form of markdown snippets grouped into spaces."</span>
            <span>"Source code: "<a href="https://github.com/TheBestTvarynka/Dataans" target="_blank">"GitHub/TbeBestTvarynka/Dataans"</a></span>
            <hr style="width: 80%" />
            <div class="horizontal">
                <button class="button_ok" on:click=open_config_file>"Edit config file"</button>
                <button
                    class="tool"
                    title="Open config file location"
                    on:click=open_config_file_folder
                >
                    <img alt="edit note" src="/public/icons/folder.png" />
                </button>
                {move || if is_autostart_enabled.get() {view! {
                    <button class="button_ok" on:click=disable_autostart title="Disable autostart">"Disable autostart"</button>
                }} else {view! {
                    <button class="button_ok" on:click=enable_autostart  title="Enable autostart">"Enable autostart"</button>
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
                                    <img alt="edit note" src="/public/icons/folder.png" />
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
            <div class="horizontal">
                <select
                    on:change=move |ev: leptos::ev::Event| {
                        let select: HtmlSelectElement = ev.target().unwrap().unchecked_into();
                        set_export_option.set(NotesExportOption::from_str(&select.value()));
                    }
                    prop:value=move || export_option.get().to_string()
                >
                    {NotesExportOption::variants().into_iter().map(|export_option| view! {
                        <option value=export_option.to_string()>{export_option.pretty()}</option>
                    }).collect_view()}
                </select>
                <select
                    on:change=move |ev: leptos::ev::Event| {
                        let select: HtmlSelectElement = ev.target().unwrap().unchecked_into();
                        set_export_format.set(ExportFormat::from_str(&select.value()));
                    }
                    prop:value=move || export_format.get().to_string()
                >
                    {ExportFormat::variants().into_iter().map(|export_format| view! {
                        <option value=export_format.to_string()>{export_format.pretty()}</option>
                    }).collect_view()}
                </select>
                <button class="button_ok" on:click=export_data>"Export"</button>
                <Suspense
                    fallback=move || view! { <span>"Exporting...."</span> }
                >
                    {move || export_result.get()
                        .flatten()
                        .map(|backup_path| view! {
                            <button class="button_ok" on:click=move |_| open_exported_data_dir(backup_path.clone())>
                                "Open backup location"
                            </button>
                        })}
                </Suspense>
            </div>
        </div>
    }
}
