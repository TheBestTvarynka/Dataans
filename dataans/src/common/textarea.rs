use leptos::web_sys::KeyboardEvent;
use leptos::*;
use wasm_bindgen::JsCast;

use crate::backend::load_clipboard_image;

#[component]
pub fn TextArea(
    id: String,
    text: Signal<String>,
    #[prop(into)] set_text: Callback<String, ()>,
    key_down: impl Fn(KeyboardEvent) + 'static,
) -> impl IntoView {
    let (disabled, set_disabled) = create_signal(false);
    let ref_input = create_node_ref::<html::Textarea>();

    create_effect(move |_| {
        if let Some(ref_input) = ref_input.get() {
            let _ = ref_input.on_mount(|input| {
                if let Err(err) = input.focus() {
                    warn!("Can not focus TextArea: {:?}", err);
                }
            });
        }
    });

    let elem_id = id.clone();
    let paste_handler = move |e: leptos::ev::Event| {
        let ev = e
            .dyn_into::<web_sys::ClipboardEvent>()
            .expect("Event -> ClipboardEvent should not fail");
        if let Some(clipboard_data) = ev.clipboard_data() {
            let types = clipboard_data.types();
            let len = types.length();
            if (0..len).any(|type_index| {
                let ty = types
                    .get(type_index)
                    .as_string()
                    .expect("MIME type JsValue should be string");
                ty.to_ascii_lowercase().contains("files")
            }) {
                ev.prevent_default();
                let mut text = text.get();
                let id = elem_id.clone();
                spawn_local(async move {
                    let image_path = load_clipboard_image().await;

                    let text_area = document().get_element_by_id(&id).expect("Dom element should present");
                    let text_area = text_area
                        .dyn_into::<web_sys::HtmlTextAreaElement>()
                        .expect("Element should be textarea");

                    if let Some(start) = text_area.selection_start().expect("selection start error") {
                        let start = start as usize;
                        text = format!("{} ![]({}){}", &text[0..start], &image_path, &text[start..]);
                    } else {
                        text.push_str("![](");
                        text.push_str(&image_path);
                        text.push(')');
                    }

                    set_text.call(text);
                    set_disabled.set(false);
                });
            } else {
                info!("No images to upload.");
            }
        } else {
            info!("No clipboard data.");
        }
    };

    let elem_id = id.clone();
    let text_editing_keybindings = move |event: KeyboardEvent| {
        if let Some(text) = text.try_get() {
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
        }
    };

    let style = move || {
        #[cfg(windows_is_host_os)]
        {
            // WARN: This CSS property is experimental and supported only in WebView2 which is used on Windows. More info:
            // https://developer.mozilla.org/en-US/docs/Web/CSS/field-sizing#browser_compatibility
            "field-sizing: content".to_owned()
        }
        #[cfg(not(windows_is_host_os))]
        {
            let lines_amount = text.get().split('\n').count();
            format!("height: {:.2?}em", lines_amount.max(1) as f32 * 1.3)
        }
    };

    view! {
        <div class="resizable-textarea">
            <textarea
                id=id.clone()
                _ref=ref_input
                type="text"
                placeholder="Type a note..."
                class="resizable-textarea-textarea"
                style=style
                on:input=move |ev| set_text.call(event_target_value(&ev))
                on:keydown=move |ev| {
                    key_down(ev.clone());
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

type TextFormatFn = &'static dyn Fn(String, String, String, u32) -> (String, Option<(u32, u32)>);

fn get_text_format_fn(event: KeyboardEvent) -> Option<TextFormatFn> {
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
