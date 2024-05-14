use common::Note as NoteData;
use leptos::*;
use markdown::mdast::{Node, Text};
use markdown::ParseOptions;

use crate::notes::md_node::render_md_node;

#[component]
pub fn Note<'text>(note: NoteData<'text>) -> impl IntoView {
    let md = markdown::to_mdast(note.text.as_ref(), &ParseOptions::default()).unwrap_or_else(|_| {
        Node::Text(Text {
            value: "Can not parse MD message".into(),
            position: None,
        })
    });
    debug!("{:?}", md);

    render_md_node(&md)
}
