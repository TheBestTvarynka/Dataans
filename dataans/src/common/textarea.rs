use std::sync::LazyLock;

use leptos::html;
use leptos::prelude::*;
use leptos::tachys::html::event::ClipboardEvent;
use leptos::task::spawn_local;
use leptos::web_sys::KeyboardEvent;
use regex::Regex;
use wasm_bindgen::JsCast;

use crate::backend::file::load_clipboard_image;

#[component]
pub fn TextArea(
    id: String,
    text: Signal<String>,
    #[prop(into)] set_text: Callback<(String,), ()>,
    key_down: impl Fn(KeyboardEvent) + 'static,
) -> impl IntoView {
    let toaster = leptoaster::expect_toaster();

    let (disabled, set_disabled) = signal(false);
    let ref_input = NodeRef::<html::Textarea>::new();

    Effect::new(move |_| {
        ref_input.on_load(|input| {
            if let Err(err) = input.focus() {
                warn!("Can not focus TextArea: {err:?}");
            }
        });
    });

    let elem_id = id.clone();
    let paste_handler = move |e: ClipboardEvent| {
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
                let toaster = toaster.clone();

                ev.prevent_default();
                let mut text = text.get();
                let id = elem_id.clone();
                spawn_local(async move {
                    let image = try_exec!(load_clipboard_image().await, "Failed to load clipboard image", toaster);
                    let image_path = try_exec!(
                        image.path.to_str().ok_or("use UTF-8 valid paths"),
                        "Image path is not valid UTF-8 string",
                        toaster
                    );

                    let text_area = document().get_element_by_id(&id).expect("Dom element should present");
                    let text_area = text_area
                        .dyn_into::<web_sys::HtmlTextAreaElement>()
                        .expect("Element should be textarea");

                    if let Some(start) = text_area.selection_start().expect("selection start error") {
                        let start = start as usize;
                        text = format!("{} ![]({}){}", &text[0..start], &image_path, &text[start..]);
                    } else {
                        text.push_str("![](");
                        text.push_str(image_path);
                        text.push(')');
                    }

                    set_text.run((text,));
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
        if let Some(format_fn) = get_text_format_fn(event) {
            let text_area = document()
                .get_element_by_id(&elem_id)
                .expect("Dom element should present");
            let text_area = text_area
                .dyn_into::<web_sys::HtmlTextAreaElement>()
                .expect("Element should be textarea");
            let text = text_area.value();

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
            text_area.set_value(&text);
            set_text.run((text,));
            if let Some((selection_start, selection_end)) = selection {
                if let Err(err) = text_area.set_selection_start(Some(selection_start)) {
                    error!("{err:?}");
                }
                if let Err(err) = text_area.set_selection_end(Some(selection_end)) {
                    error!("{err:?}");
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
                node_ref=ref_input
                placeholder="Type a note..."
                class="resizable-textarea-textarea"
                style=style
                on:input=move |ev| set_text.run((event_target_value(&ev),))
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
                format!("{pre_text}[{selected_text}](url){after_text}"),
                Some((selection_start, selection_start + 3 /* "url" */)),
            )
        })
    } else if event.ctrl_key() && event.key() == "b" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection_start = start + 2 /* "**" */;
            let selection_end = selection_start + selected_text.len() as u32;
            (
                format!("{pre_text}**{selected_text}**{after_text}"),
                Some((selection_start, selection_end)),
            )
        })
    } else if event.ctrl_key() && event.key() == "i" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection_start = start + 1 /* "_" */;
            let selection_end = selection_start + selected_text.len() as u32;
            (
                format!("{pre_text}_{selected_text}_{after_text}"),
                Some((selection_start, selection_end)),
            )
        })
    } else if event.ctrl_key() && event.shift_key() && event.key() == "M" {
        Some(&move |pre_text, selected_text, after_text, start| {
            let selection_start = start + 1 /* "`" */;
            let selection_end = selection_start + selected_text.len() as u32;
            (
                format!("{pre_text}`{selected_text}`{after_text}"),
                Some((selection_start, selection_end)),
            )
        })
    } else if event.shift_key() && event.key() == "Enter" {
        event.prevent_default();

        Some(
            &move |pre_text, selected_text, after_text, _start| match parse_prev_line(&pre_text) {
                LineType::None { trimmed } => {
                    let current = pre_text.len() as u32 + 1 + trimmed.len() as u32;
                    (
                        format!("{pre_text}\n{trimmed}{selected_text}{after_text}"),
                        Some((current, current)),
                    )
                }
                LineType::UnorderedList { trimmed, marker } => {
                    let current = pre_text.len() as u32 + 3 + trimmed.len() as u32;
                    (
                        format!("{pre_text}\n{trimmed}{marker} {selected_text}{after_text}"),
                        Some((current, current)),
                    )
                }
                LineType::OrderedList {
                    trimmed,
                    number: current_number,
                } => {
                    let number = (current_number + 1).to_string();
                    let current = pre_text.len() as u32 + 3 + trimmed.len() as u32 + number.len() as u32;
                    let after_text = if !after_text.is_empty()
                        && after_text[1 + trimmed.len()..].starts_with(&format!("{number}. "))
                    {
                        increment_next_items(&after_text[1..], current_number + 1, trimmed)
                    } else {
                        after_text
                    };

                    (
                        format!("{pre_text}\n{trimmed}{number}. {selected_text}{after_text}"),
                        Some((current, current)),
                    )
                }
            },
        )
    } else if !event.ctrl_key() && !event.alt_key() && (event.key() == "Tab" || event.code() == "Tab") {
        event.prevent_default();

        if event.shift_key() {
            // Decrement indentation.
            Some(&move |pre_text, selected_text, after_text, _start| {
                let (pre_text, lines, after_text) =
                    select_lines_for_indentation(&pre_text, &selected_text, &after_text);
                let mut lines = lines.lines();
                let mut formatted = lines.next().map(decrement_indentation).unwrap_or_default().to_owned();

                lines.for_each(|line| {
                    formatted.push('\n');
                    formatted.push_str(&decrement_indentation(line));
                });

                (
                    format!("{pre_text}{formatted}{after_text}"),
                    Some((pre_text.len() as u32, (pre_text.len() + formatted.len()) as u32)),
                )
            })
        } else {
            // Increment indentation.
            Some(&move |pre_text, selected_text, after_text, _start| {
                let (pre_text, lines, after_text) =
                    select_lines_for_indentation(&pre_text, &selected_text, &after_text);
                let mut lines = lines.lines();
                let mut formatted = lines
                    .next()
                    .map(|line| format!("  {line}"))
                    .unwrap_or_default()
                    .to_owned();

                lines.for_each(|line| {
                    formatted.push('\n');
                    formatted.push_str("  ");
                    formatted.push_str(line);
                });

                (
                    format!("{pre_text}{formatted}{after_text}"),
                    Some((pre_text.len() as u32, (pre_text.len() + formatted.len()) as u32)),
                )
            })
        }
    } else if event.key() == "`" {
        event.prevent_default();

        Some(&move |pre_text, selected_text, after_text, start| {
            let selection_start = start + 1;
            if pre_text.ends_with("``") {
                // `get_prev_line` returns `None` only when `\n` is the last char of the string slice.
                // But it is impossible because `pre_text` ends with "``" (if statement above).
                // Thus, it is safe to unwrap here.
                let prev_line = get_prev_line(&pre_text).expect("'\n' should not be the last char in `pre_text`");
                (
                    format!("{pre_text}`{selected_text}\n{prev_line}`{after_text}"),
                    Some((selection_start, selection_start)),
                )
            } else {
                (
                    format!("{pre_text}`{selected_text}{after_text}"),
                    Some((selection_start, selection_start)),
                )
            }
        })
    } else {
        None
    }
}

fn decrement_indentation(line: &str) -> String {
    let tab_size = 2.min(line.len());

    let mut i = 0;

    while i < tab_size {
        if !char::from(line.as_bytes()[i]).is_ascii_whitespace() {
            break;
        }

        i += 1;
    }

    line[i..].to_string()
}

fn select_lines_for_indentation<'a>(
    pre_text: &'a str,
    selected_text: &'a str,
    after_text: &'a str,
) -> (&'a str, String, &'a str) {
    let start = pre_text.rfind('\n');
    let end = after_text.find('\n');

    match (start, end) {
        (Some(start), Some(end)) => {
            let mut lines = String::new();

            if start + 1 < pre_text.len() {
                lines.push_str(&pre_text[start + 1..]);
            }

            lines.push_str(selected_text);
            lines.push_str(&after_text[0..end]);

            (&pre_text[0..start + 1], lines, &after_text[end..])
        }
        // The user selected from some part of the text to the end of text (to the last line)
        // There are some lines before the start of the selection.
        (Some(start), None) => {
            let mut lines = String::new();

            if start + 1 < pre_text.len() {
                lines.push_str(&pre_text[start + 1..]);
            }

            lines.push_str(selected_text);
            lines.push_str(after_text);

            (&pre_text[0..start + 1], lines, "")
        }
        // The user selected from the first line to some part of the text (there are some lines after the end of the selection).
        (None, Some(end)) => {
            let mut lines = String::new();
            lines.push_str(pre_text);
            lines.push_str(selected_text);
            lines.push_str(&after_text[0..end]);

            ("", lines, &after_text[end..])
        }
        // The user started selection from the first line to the end of text (to the last line).
        (None, None) => {
            let mut lines = String::with_capacity(pre_text.len() + selected_text.len() + after_text.len());
            lines.push_str(pre_text);
            lines.push_str(selected_text);
            lines.push_str(after_text);

            ("", lines, "")
        }
    }
}

enum LineType<'a> {
    UnorderedList { trimmed: &'a str, marker: char },
    OrderedList { trimmed: &'a str, number: u32 },
    None { trimmed: &'a str },
}

static ORDERED_LIST_PATTERN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^(\d+)\. ").unwrap());

fn increment_next_items(after_text: &str, mut number: u32, padding: &str) -> String {
    let mut result = String::new();

    let mut lines = after_text.lines();
    for line in &mut lines {
        let pattern = format!("{padding}{number}. ");

        if line.starts_with(&pattern) {
            result.push_str(&format!("\n{padding}{}. {}", number + 1, &line[pattern.len()..]));
            number += 1;
        } else {
            result.push('\n');
            result.push_str(line);
            break;
        }
    }

    lines.fold(result, |mut result, line| {
        result.push('\n');
        result.push_str(line);
        result
    })
}

fn parse_line<'a>(line: &'a str) -> LineType<'a> {
    let trimmed_line = line.trim_start();
    let (trimmed, line) = line.split_at(line.len() - trimmed_line.len());

    if line.starts_with("* ") {
        LineType::UnorderedList { trimmed, marker: '*' }
    } else if line.starts_with("- ") {
        LineType::UnorderedList { trimmed, marker: '-' }
    } else if line.starts_with("+ ") {
        LineType::UnorderedList { trimmed, marker: '+' }
    } else if let Some(number) = ORDERED_LIST_PATTERN.find(line) {
        let number = number.as_str();
        number[0..number.len() - 2 /* ". " */]
            .parse::<u32>()
            .map(|number| LineType::OrderedList { trimmed, number })
            .unwrap_or(LineType::None { trimmed })
    } else {
        LineType::None { trimmed }
    }
}

fn get_prev_line(pre_text: &str) -> Option<&str> {
    Some(if let Some(start) = pre_text.rfind('\n') {
        if start == pre_text.len() - 1 {
            return None;
        }

        &pre_text[start + 1..]
    } else {
        pre_text
    })
}

fn parse_prev_line<'a>(pre_text: &'a str) -> LineType<'a> {
    let Some(prev_line) = get_prev_line(pre_text) else {
        return LineType::None { trimmed: "" };
    };

    parse_line(prev_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decrement_indentation() {
        assert_eq!(
            decrement_indentation("        LineType::None { trimmed }"),
            "      LineType::None { trimmed }",
        );

        assert_eq!(
            decrement_indentation("  LineType::None { trimmed }"),
            "LineType::None { trimmed }",
        );

        assert_eq!(
            decrement_indentation(" LineType::None { trimmed }"),
            "LineType::None { trimmed }",
        );

        assert_eq!(
            decrement_indentation("LineType::None { trimmed }"),
            "LineType::None { trimmed }",
        );
    }
}
