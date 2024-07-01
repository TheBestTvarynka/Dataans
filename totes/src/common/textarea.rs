use js_sys::{ArrayBuffer, Uint8Array};
use leptos::web_sys::KeyboardEvent;
use leptos::*;
use wasm_bindgen::JsCast;

use crate::backend::save_image;

#[component]
pub fn TextArea(
    id: String,
    text: ReadSignal<String>,
    #[prop(into)] set_text: Callback<String, ()>,
    #[prop(into)] key_down: Callback<KeyboardEvent, ()>,
) -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);

    let elem_id = id.clone();
    let paste_handler = move |e: leptos::ev::Event| {
        let ev = e
            .dyn_into::<web_sys::ClipboardEvent>()
            .expect("Event -> ClipboardEvent should not fail");
        if let Some(clipboard_data) = ev.clipboard_data() {
            let items = clipboard_data.items();
            for index in 0..items.length() {
                let item = items.get(index).expect("DataTransferItem should present");

                if item.kind() == "file" && item.type_().starts_with("image/") {
                    if let Some(file) = item.get_as_file().expect("get_as_fail should not fail") {
                        ev.prevent_default();

                        let image_raw_data = file.slice().expect("File reading should not fail");
                        let file_name = file.name();
                        let mut text = text.get();
                        let id = elem_id.clone();

                        set_disabled.set(true);
                        spawn_local(async move {
                            let image_raw_data = wasm_bindgen_futures::JsFuture::from(image_raw_data.array_buffer())
                                .await
                                .expect("File reading should not fail");

                            let image_raw_data = image_raw_data
                                .dyn_into::<ArrayBuffer>()
                                .expect("Expected an ArrayBuffer");
                            let image_raw_data = Uint8Array::new(&image_raw_data);

                            let mut image_bytes = vec![0; image_raw_data.length() as usize];
                            image_raw_data.copy_to(image_bytes.as_mut_slice());

                            let path = save_image(&file_name, &image_bytes).await;

                            let text_area = document().get_element_by_id(&id).expect("Dom element should present");
                            let text_area = text_area
                                .dyn_into::<web_sys::HtmlTextAreaElement>()
                                .expect("Element should be textarea");

                            if let Some(start) = text_area.selection_start().expect("selection start error") {
                                let start = start as usize;
                                text = format!("{}\n![]({}){}", &text[0..start], &path, &text[start..]);
                            } else {
                                text.push_str(" ![](");
                                text.push_str(&path);
                                text.push_str(")");
                            }

                            set_text.call(text);
                            set_disabled.set(false);
                        });
                    } else {
                        warn!("No file :(");
                    }
                }
            }
        }
    };

    view! {
        <div class="resizable-textarea">
            <textarea
                id=id.clone()
                type="text"
                placeholder="Type a note..."
                class="resizable-textarea-textarea"
                // WARN: This CSS property is experimental.
                // https://developer.mozilla.org/en-US/docs/Web/CSS/field-sizing#browser_compatibility
                style="field-sizing: content"
                on:input=move |ev| set_text.call(event_target_value(&ev))
                on:keydown=move |ev| key_down.call(ev)
                on:paste=paste_handler
                prop:value=move || text.get()
                disabled=move || disabled.get()
            >
                {text.get_untracked()}
            </textarea>
        </div>
    }
}
