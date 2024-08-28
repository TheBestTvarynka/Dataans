use common::note::File;
use leptos::*;

use crate::backend::file::{open, reveal};

#[component]
pub fn File(file: File, edit_mode: bool, #[prop(into)] remove_file: Callback<File, ()>) -> impl IntoView {
    let file_data = file.clone();
    let file_path = file.path.clone();

    let open_file = move |_| {
        let file = file_path.clone();
        spawn_local(async move {
            open(&file).await;
        });
    };

    view! {
        <div class="files-file">
            {if edit_mode {
                view! {
                    <img alt="" src="/public/icons/cancel.png" class="files-file-cancel" on:click=move |_| remove_file.call(file_data.clone()) />
                    <img src="/public/icons/file.png" alt="" class="files-file-icon" />
                }
            } else {
                let file_path = file.path;
                let reveal_file = move |_| {
                    let file = file_path.clone();
                    spawn_local(async move {
                        reveal(&file).await;
                    });
                };

                view! {
                    <img alt="" src="/public/icons/folder.png" class="files-file-cancel" on:click=reveal_file />
                    <img src="/public/icons/file.png" alt="" class="files-file-icon" />
                }
            }}
            <span on:click=open_file>{file.name.clone()}</span>
        </div>
    }
}

#[component]
pub fn Files(files: Vec<File>, edit_mode: bool, #[prop(into)] remove_file: Callback<File, ()>) -> impl IntoView {
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
