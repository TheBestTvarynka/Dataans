use common::note::File;
use leptos::*;

#[component]
pub fn File(file: File, #[prop(into)] remove_file: Callback<File, ()>) -> impl IntoView {
    let file_data = file.clone();

    view! {
        <div class="files-file">
            <img alt="" src="/public/icons/cancel.png" class="files-file-cancel" on:click=move |_| remove_file.call(file_data.clone()) />
            <img src="/public/icons/file.png" alt="" class="files-file-icon" />
            <span>{file.name.clone()}</span>
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
