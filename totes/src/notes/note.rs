use common::note::Note as NoteData;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;
use time::OffsetDateTime;

use crate::notes::md_node::render_md_node;

#[allow(clippy::needless_lifetimes)]
#[component]
pub fn Note<'text>(note: NoteData<'text>) -> impl IntoView {
    let md = markdown::to_mdast(note.text.as_ref(), &ParseOptions::gfm()).unwrap_or_else(|_| {
        Node::Text(Text {
            value: "Can not parse MD message".into(),
            position: None,
        })
    });

    view! {
        <div class="note-container">
            <div class="note-meta">
                <div class="horizontal note-tools">
                    <button
                        class="tool"
                        title="Edit note"
                    >
                        <img alt="change space name" src="/public/icons/edit-space.svg" />
                    </button>
                    <button
                        class="tool"
                        title="Delete note"
                    >
                        <img alt="delete space" src="/public/icons/delete-space.png" />
                    </button>
                </div>
                <div class="center-span">
                    <span class="note-time">{format_date(note.created_at.as_ref())}</span>
                </div>
            </div>
            {render_md_node(&md)}
        </div>
    }
}

fn format_date(date: &OffsetDateTime) -> String {
    format!("{}:{} {}/{}/{}", date.hour(), date.minute(), date.day(), date.month(), date.year())
}
