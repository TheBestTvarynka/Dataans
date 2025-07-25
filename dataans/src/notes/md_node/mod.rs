mod code_block;
mod inline_code;
mod list_item;

use std::path::Path;

pub use inline_code::InlineCode;
use leptos::html::HtmlElement;
use leptos::prelude::*;
use leptos::task::spawn_local;
use markdown::mdast::Node;

use self::code_block::CodeBlock;
use self::list_item::ListItem;
use crate::backend::convert_file_src;
use crate::backend::file::open;

pub fn render_md_node(node: &Node, base_path: &str) -> HtmlElement<AnyView> {
    match node {
        Node::Root(root) => view! {
            <div class="note">
                {root.children
                    .iter()
                    .map(|n| render_md_node(n, base_path))
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Paragraph(paragraph) => view! {
            <span class="paragraph">
                {paragraph.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </span>
        }
        .into_any(),
        Node::ThematicBreak(_) => view! { <br class="br" /> }.into_any(),
        Node::Heading(heading) => {
            let inner = heading.children.iter().map(|n| render_md_node(n, base_path)).collect::<Vec<_>>();
            match heading.depth {
                1 => view! {
                    <h1>{inner}</h1>
                }
                .into_any(),
                2 => view! {
                    <h2>{inner}</h2>
                }
                .into_any(),
                3 => view! {
                    <h3>{inner}</h3>
                }
                .into_any(),
                4 => view! {
                    <h4>{inner}</h4>
                }
                .into_any(),
                5 => view! {
                    <h5>{inner}</h5>
                }
                .into_any(),
                6 => view! {
                    <h6>{inner}</h6>
                }
                .into_any(),
                // Should never be reached but let it be here just in case.
                n => view! {
                    <span>{format!("Heading with depth={n} is not supported!")}</span>
                }
                .into_any(),
            }
        }
        Node::InlineCode(code) => view! {
            <span>
                <InlineCode code={code.value.clone()} />
            </span>
        }
        .into_any(),
        Node::Text(text) => view! { <span class="text">{&text.value}</span> }.into_any(),
        Node::Delete(delete) => view! {
            <s>
                {delete.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </s>
        }
        .into_any(),
        Node::Emphasis(emphasis) => view! {
            <em>
                {emphasis.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </em>
        }
        .into_any(),
        Node::Strong(strong) => view! {
            <b>
                {strong.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </b>
        }
        .into_any(),
        Node::Blockquote(quote) => view! {
            <div class="quote">
                {quote.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Link(link) => view! {
            <a class="link" href=&link.url target="_blank">
                {link.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect::<Vec<_>>()}
            </a>
        }
        .into_any(),
        Node::List(list) => {
            if list.ordered {
                view! {
                    <ol class="list" start=list.start.unwrap_or(1)>
                        {list.children
                            .iter()
                            .map(|n| render_md_node(n,  base_path))
                            .collect::<Vec<_>>()}
                    </ol>
                }
                .into_any()
            } else {
                view! {
                    <ul class="list">
                        {list.children
                            .iter()
                            .map(|n| render_md_node(n,  base_path))
                            .collect::<Vec<_>>()}
                    </ul>
                }
                .into_any()
            }
        }
        Node::ListItem(list_item) => view! {
            <div>
                <ListItem list_item=list_item.clone() base_path />
            </div>
        }
        .into_any(),
        Node::Image(image) => {
            let image_path = image.url.clone();
            let open_image = move |_| {
                let path = image_path.clone();
                spawn_local(async move {
                    open(Path::new(&path)).await;
                })
            };
            view! {
                <img src=convert_file_src(&image.url, base_path) alt=image.alt.clone() class="note-image" on:click=open_image />
            }
        }
        .into_any(),
        Node::Table(table) => view! {
            <table class="table">
                {
                    if let Node::TableRow(header_row) = table.children.first().unwrap() {
                        view! {
                            <tr class="table-header">
                                {header_row.children
                                    .iter()
                                    .map(|n| render_md_node(n,  base_path))
                                    .collect_view()}
                            </tr>
                        }
                        .into_any()
                    } else {
                        view! {
                            <span>"The first Table children should be TableRow"</span>
                        }
                        .into_any()
                    }
                }
                {
                    table.children[1..]
                        .iter()
                        .map(|n| render_md_node(n,  base_path))
                        .collect_view()
                }
            </table>
        }
        .into_any(),
        Node::TableRow(row) => view! {
            <tr class="table-row">
                {row.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect_view()}
            </tr>
        }
        .into_any(),
        Node::TableCell(cell) => view! {
            <td class="table-cell">
                {cell.children
                    .iter()
                    .map(|n| render_md_node(n,  base_path))
                    .collect_view()}
            </td>
        }
        .into_any(),
        Node::Code(code) => {
            let lang = code.lang.clone().unwrap_or_else(|| String::from("txt"));
            view! {
                <div>
                    <CodeBlock code=code.value.clone() lang/>
                </div>
            }
            .into_any()
        }
        v => view! { <span>{format!("{v:?} is not supported")}</span> }.into_any(),
    }
}
