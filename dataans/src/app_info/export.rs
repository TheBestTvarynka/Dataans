use std::path::PathBuf;

use common::export::SchemaVersion;
use common::{DataExportConfig, NotesExportOption};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::backend::export::export_data;
use crate::backend::file::open;

#[component]
pub fn Export() -> impl IntoView {
    let (export_config, set_export_config) = create_signal(DataExportConfig::default());
    let (backup_dir, set_backup_dir) = create_signal(None);

    let export_data_action = Action::new(move |export_config: &DataExportConfig| {
        let export_config = export_config.clone();
        async move {
            let backup_dir = export_data(export_config).await;
            set_backup_dir.set(Some(backup_dir));
        }
    });

    let open_backup_folder = move |path: PathBuf| {
        spawn_local(async move {
            set_backup_dir.set(None);
            open(&path).await;
        });
    };

    view! {
        <div class="horizontal">
            {move || {
                let export_config = export_config.get();
                view! {
                    <select
                        class="input"
                        on:change=move |ev: leptos::ev::Event| {
                            let select: HtmlSelectElement = ev.target().unwrap().unchecked_into();
                            set_export_config.set(DataExportConfig::_from_str(&select.value()));
                        }
                    >
                        {DataExportConfig::variants().iter().map(|export_option| view! {
                            <option
                                value=export_option.variant_name()
                                selected={export_option.variant_name() == export_config.variant_name()}
                            >
                                {export_option.variant_name()}
                            </option>
                        }).collect_view()}
                    </select>
                }
            }}
            {move || match export_config.get() {
                DataExportConfig::Md(notes_export_option) => view! {
                    <select
                        class="input"
                        on:change=move |ev: leptos::ev::Event| {
                            let select: HtmlSelectElement = ev.target().unwrap().unchecked_into();
                            set_export_config.set(DataExportConfig::Md(NotesExportOption::_from_str(&select.value())));
                        }
                    >
                        {NotesExportOption::variants().iter().map(|export_option| view! {
                            <option
                                value=export_option.variant_name()
                                selected=export_option.variant_name() == notes_export_option.variant_name()
                            >
                                {export_option.pretty()}
                            </option>
                        }).collect_view()}
                    </select>
                },
                DataExportConfig::Json(schema_version) => view! {
                    <select
                        class="input"
                        on:change=move |ev: leptos::ev::Event| {
                            let select: HtmlSelectElement = ev.target().unwrap().unchecked_into();
                            set_export_config.set(DataExportConfig::Json(SchemaVersion::_from_str(&select.value())));
                        }
                    >
                        {SchemaVersion::variants().iter().map(|version| view! {
                            <option
                                value=version.variant_name()
                                selected=version.variant_name() == schema_version.variant_name()
                            >
                                {schema_version.to_string()}
                            </option>
                        }).collect_view()}
                    </select>
                },
            }}
            <button class="button_cancel" on:click=move |_| export_data_action.dispatch(export_config.get())>"Export"</button>
            <Show when=move || backup_dir.get().is_some()>
                {move || {
                    let backup_dir = backup_dir.get().unwrap();
                    view! {
                        <button
                            class="tool"
                            title="Open backup folder"
                            on:click=move |_| open_backup_folder(backup_dir.clone())
                        >
                            <img alt="edit note" src="/public/icons/folder.png" />
                        </button>
                    }
                }}
            </Show>
        </div>
    }
}
