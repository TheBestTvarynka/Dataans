use common::note::File;
use js_sys::{ArrayBuffer, Uint8Array};
use leptos::{component, spawn_local, view, Callable, Callback, IntoView, ReadSignal, SignalGet};
use uuid::Uuid;
use wasm_bindgen::JsCast;
use web_sys::{Blob, HtmlInputElement};

#[component]
pub fn Attachment(
    id: String,
    files: ReadSignal<Vec<File>>,
    #[prop(into)] set_files: Callback<Vec<File>, ()>,
) -> impl IntoView {
    let handle_files_upload = move |ev: leptos::ev::Event| {
        let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
        let mut attached_files = files.get();

        if let Some(files) = input.files() {
            let files = (0..files.length())
                .map(|index| {
                    let file = files.get(index).unwrap();
                    let blob = file.slice().expect("File reading should not fail");
                    let name = file.name();
                    let id = Uuid::new_v4();

                    async move { (name.clone(), id, upload_file(blob, name, id).await) }
                })
                .collect::<Vec<_>>();

            let files = futures::future::join_all(files);
            spawn_local(async move {
                let files = files
                    .await
                    .into_iter()
                    .map(|(name, id, path)| File {
                        id,
                        name,
                        path: path.into(),
                    })
                    .collect::<Vec<_>>();
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
            <input id=format!("attachment_{}", id) type="file" multiple=true style="display: none" on:input=move |ev| handle_files_upload(ev) />
        </button>
    }
}

// Returns path to the uploaded file.
async fn upload_file(blob: Blob, name: String, id: Uuid) -> String {
    let file_raw_data = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
        .await
        .expect("File reading should not fail");

    let file_raw_data = file_raw_data
        .dyn_into::<ArrayBuffer>()
        .expect("Expected an ArrayBuffer");
    let file_raw_data = Uint8Array::new(&file_raw_data);

    let mut file_bytes = vec![0; file_raw_data.length() as usize];
    file_raw_data.copy_to(file_bytes.as_mut_slice());

    crate::backend::file::upload_file(id, &name, &file_bytes).await
}
