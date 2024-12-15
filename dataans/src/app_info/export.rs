use common::export::SchemaVersion;
use common::{DataExportConfig, NotesExportOption};
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

use crate::backend::export::export_data;
// use crate::backend::file::open;

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

    view! {
        <div>
            {move || {
                let export_config = export_config.get();
                view! {
                    <select
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
            <button class="button_ok" on:click=move |_| export_data_action.dispatch(export_config.get())>"Export"</button>
            {move || if let Some(backup_dir) = backup_dir.get() {view! {
                <span>{backup_dir.to_str().expect("UTF-8 valid path").to_string()}</span>
            }} else { view! { <span /> }}}
        </div>
    }
}
