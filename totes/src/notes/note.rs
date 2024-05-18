use common::note::Note as NoteData;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;

use crate::notes::md_node::render_md_node;

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
            {render_md_node(&md)}
            <div class="note-meta">
                <span class="note-time">{note.created_at.as_ref().time().to_string()}</span>
            </div>
        </div>
    }
}
