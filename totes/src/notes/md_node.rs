use leptos::html::AnyElement;
use leptos::*;
use markdown::mdast::Node;

pub fn render_md_node(node: &Node) -> HtmlElement<AnyElement> {
    match node {
        Node::Root(root) => view! {
            <div class="note">
                {root.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Paragraph(paragraph) => view! {
            <p class="paragraph">
                {paragraph.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </p>
        }
        .into_any(),
        Node::ThematicBreak(_) => view! { <br class="br" /> }.into_any(),
        Node::Heading(heading) => {
            let inner = heading.children.iter().map(render_md_node).collect::<Vec<_>>();
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
                // Note: should never be reached but let it be here just in case.
                n => view! {
                    <span>{format!("Heading with depth={} is not supported!", n)}</span>
                }
                .into_any(),
            }
        }
        Node::InlineCode(code) => {
            let code_value = code.value.clone();
            view! {
                <span class="incline-code" on:click=move |_| {
                    if let Some(clipboard) = window().navigator().clipboard() {
                        let _ = clipboard.write_text(&code_value);
                    } else {
                        error!("clipboard is not defined.")
                    }
                }>{&code.value}</span>
            }
            .into_any()
        }
        Node::Text(text) => view! { <span class="text">{&text.value}</span> }.into_any(),
        Node::Delete(delete) => view! {
            <s>
                {delete.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </s>
        }
        .into_any(),
        Node::Emphasis(emphasis) => view! {
            <em>
                {emphasis.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </em>
        }
        .into_any(),
        Node::Strong(strong) => view! {
            <b>
                {strong.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </b>
        }
        .into_any(),
        Node::BlockQuote(quote) => view! {
            <div class="quote">
                {quote.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Link(link) => view! {
            <a class="link" href={&link.url} target="popup">
                {link.children
                    .iter()
                    .map(render_md_node)
                    .collect::<Vec<_>>()}
            </a>
        }
        .into_any(),
        Node::List(list) => {
            if list.ordered {
                view! {
                    <ol class="list" start={list.start.unwrap_or(1)}>
                        {list.children
                            .iter()
                            .map(render_md_node)
                            .collect::<Vec<_>>()}
                    </ol>
                }
                .into_any()
            } else {
                view! {
                    <ul class="list">
                        {list.children
                            .iter()
                            .map(render_md_node)
                            .collect::<Vec<_>>()}
                    </ul>
                }
                .into_any()
            }
        }
        Node::ListItem(list_item) => view! {
            <li>
                {list_item.children
                    .iter()
                    .map(render_md_node)
                    .collect_view()}
            </li>
        }
        .into_any(),
        v => view! { <span>{format!("{:?} is not supported", v)}</span> }.into_any(),
    }
}
