use common::note::{File, FileStatus};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::backend::file::{open, reveal};

#[component]
pub fn File(file: File, edit_mode: bool, #[prop(into)] remove_file: Callback<(File,), ()>) -> impl IntoView {
    let file_data = file.clone();
    let file_path = file.path.clone();

    let open_file = move |_| {
        let file = file_path.clone();
        spawn_local(async move {
            open(&file).await;
        });
    };

    let icon = match file.status {
        FileStatus::ExistAndUploaded => "/public/icons/file.png",
        FileStatus::ExistAndNotUploaded => "/public/icons/file-upload.png",
        FileStatus::NotExistAndUploaded => "/public/icons/file-download.png",
        FileStatus::NotExistAndNotUploaded => "/public/icons/file-broken.png",
    };

    view! {
        <div class="files-file">
            {if edit_mode {
                view! {
                    <img alt="" title="remove file" src="/public/icons/cancel-dark.png" class="files-file-cancel" on:click=move |_| remove_file.run((file_data.clone(),)) />
                    <img src=icon alt="" class="files-file-icon" />
                }
                .into_any()
            } else {
                let file_path = file.path;
                let reveal_file = move |_| {
                    let file = file_path.clone();
                    spawn_local(async move {
                        reveal(&file).await;
                    });
                };

                view! {
                    <img alt="" title="open file location" src="/public/icons/folder-dark.png" class="files-file-cancel" on:click=reveal_file />
                    <img src=icon alt="" class="files-file-icon" />
                }
                .into_any()
            }}
            <span title="click to open the file" on:click=open_file>{file.name.clone()}</span>
        </div>
    }
}

#[component]
pub fn Files(files: Vec<File>, edit_mode: bool, #[prop(into)] remove_file: Callback<(File,), ()>) -> impl IntoView {
    view! {
        <div class="files-container">
            {move || files
                .iter()
                .map(|file| view! { <File file=file.clone() remove_file edit_mode /> })
                .collect_view()
            }
        </div>
    }
}
