use common::note::File;
use leptos::*;

use crate::backend::file::open;

#[component]
pub fn File(file: File, #[prop(into)] remove_file: Callback<File, ()>) -> impl IntoView {
    let file_data = file.clone();
    let file_path = file.path;

    let open_file = move |_| {
        let file = file_path.clone();
        spawn_local(async move {
            open(&file).await;
        });
    };

    view! {
        <div class="files-file">
            <img alt="" src="/public/icons/cancel.png" class="files-file-cancel" on:click=move |_| remove_file.call(file_data.clone()) />
            <img src="/public/icons/file.png" alt="" class="files-file-icon" />
            <span on:click=open_file>{file.name.clone()}</span>
        </div>
    }
}

#[component]
pub fn Files(files: Vec<File>, #[prop(into)] remove_file: Callback<File, ()>) -> impl IntoView {
    view! {
        <div class="files-container">
            {move || files
                .iter()
                .map(|file| view! { <File file={file.clone()} remove_file /> })
                .collect_view()
            }
        </div>
    }
}
