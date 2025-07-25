use common::note::File;
use futures::future::try_join_all;
use js_sys::{ArrayBuffer, Uint8Array};
use leptos::callback::{Callable, Callback};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{component, view, IntoView};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlInputElement};

#[component]
pub fn Attachment(
    id: String,
    files: Signal<Vec<File>>,
    #[prop(into)] set_files: Callback<Vec<File>, ()>,
) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let handle_files_upload = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        let mut attached_files = files.get();

        if let Some(files) = input.files() {
            let files = try_join_all((0..files.length()).map(|index| {
                let file = files.get(index).unwrap();
                let blob = file.slice().expect("File reading should not fail");
                let name = file.name();
                let id = Uuid::new_v4();

                async move { upload_file(blob, name, id).await }
            }));
            let toaster = toaster.clone();

            spawn_local(async move {
                let files = try_exec!(files.await, "Failed to upload files", toaster);
                attached_files.extend_from_slice(&files);
                set_files.call(attached_files);
            });
        };
    };

    view! {
        <button class="tool">
            <label for=format!("attachment_{}", id)>
                <img alt="attach file" src="/public/icons/attachment.png" />
            </label>
            <input id=format!("attachment_{}", id) type="file" multiple=true style="display: none" on:input=handle_files_upload />
        </button>
    }
}

// Returns path to the uploaded file.
async fn upload_file(blob: Blob, name: String, id: Uuid) -> Result<File, String> {
    let file_raw_data = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
        .await
        .expect("File reading should not fail");

    let file_raw_data = file_raw_data
        .dyn_into::<ArrayBuffer>()
        .expect("Expected an ArrayBuffer");
    let file_raw_data = Uint8Array::new(&file_raw_data);

    let mut file_bytes = vec![0; file_raw_data.length() as usize];
    file_raw_data.copy_to(file_bytes.as_mut_slice());

    crate::backend::file::upload_file(id, &name, &file_bytes)
        .await
        .map_err(|err| err.to_string())
}
