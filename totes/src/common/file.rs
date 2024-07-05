use common::note::File;
use leptos::*;

#[component]
pub fn File(file: File) -> impl IntoView {
    view! {
        <div class="files-file">
            <img src="/public/icons/file.png" alt="" />
            <span>{file.name.clone()}</span>
        </div>
    }
}

#[component]
pub fn Files(files: Vec<File>) -> impl IntoView {
    view! {
        <div class="files-container">
            {move || files
                .iter()
                .map(|file| view! { <File file={file.clone()} /> })
                .collect_view()
            }
        </div>
    }
}
