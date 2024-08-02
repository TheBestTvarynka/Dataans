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
        // NOTE: This code may not work on Linux. Image pasting from clipboard is a big problem for the current software industry.
        // webkit: Can't paste image data (like image/png) from the clipboard in WebKit browsers
        // https://bugs.webkit.org/show_bug.cgi?id=218519
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

    let elem_id = id.clone();
    let text_editing_keybindings = move |event: KeyboardEvent| {
        let text = text.get();
        if let Some(format_fn) = get_text_format_fn(event) {
            let text_area = document()
                .get_element_by_id(&elem_id)
                .expect("Dom element should present");
            let text_area = text_area
                .dyn_into::<web_sys::HtmlTextAreaElement>()
                .expect("Element should be textarea");

            let (text, selection) = match (
                text_area.selection_start().ok().flatten(),
                text_area.selection_end().ok().flatten(),
            ) {
                (Some(start), Some(end)) => {
                    let pre_text = text.chars().take(start as usize).collect::<String>();
                    let link_text = text
                        .chars()
                        .skip(start as usize)
                        .take((end - start) as usize)
                        .collect::<String>();
                    let after_text = text.chars().skip(end as usize).collect::<String>();

                    format_fn(pre_text, link_text, after_text, start)
                }
                (Some(position), None) | (None, Some(position)) => {
                    let pre_text = text.chars().take(position as usize).collect::<String>();
                    let after_text = text.chars().skip(position as usize).collect::<String>();

                    format_fn(pre_text, String::new(), after_text, position)
                }
                (None, None) => {
                    warn!("Empty text selection");
                    (text, None)
                }
            };
            set_text.call(text);
            if let Some((selection_start, selection_end)) = selection {
                if let Err(err) = text_area.set_selection_start(Some(selection_start)) {
                    error!("{:?}", err);
                }
                if let Err(err) = text_area.set_selection_end(Some(selection_end)) {
                    error!("{:?}", err);
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
                on:keydown=move |ev| {
                    key_down.call(ev.clone());
                    text_editing_keybindings(ev);
                }
                on:paste=paste_handler
                prop:value=move || text.get()
                disabled=move || disabled.get()
            >
                {text.get_untracked()}
            </textarea>
        </div>
    }
}

fn get_text_format_fn(
    event: KeyboardEvent,
) -> Option<&'static dyn Fn(String, String, String, u32) -> (String, Option<(u32, u32)>)> {
    if event.ctrl_key() && event.key() == "k" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection_start = start + selected_text.len() as u32 + 3 /* "[](" */;
            (
                format!("{}[{}](url){}", pre_text, selected_text, after_text),
                Some((selection_start, selection_start + 3 /* "url" */)),
            )
        })
    } else if event.ctrl_key() && event.key() == "b" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection = start + 2 /* "**" */ + selected_text.len() as u32 + 2 /* "**" */;
            (
                format!("{}**{}**{}", pre_text, selected_text, after_text),
                Some((selection, selection)),
            )
        })
    } else if event.ctrl_key() && event.key() == "i" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection = start + 1 /* "*" */ + selected_text.len() as u32 + 1 /* "*" */;
            (
                format!("{}*{}*{}", pre_text, selected_text, after_text),
                Some((selection, selection)),
            )
        })
    } else if event.ctrl_key() && event.shift_key() && event.key() == "M" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection = start + 1 /* "`" */ + selected_text.len() as u32 + 1 /* "`" */;
            (
                format!("{}`{}`{}", pre_text, selected_text, after_text),
                Some((selection, selection)),
            )
        })
    } else {
        None
    }
}
