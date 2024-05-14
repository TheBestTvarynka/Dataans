use leptos::html::AnyElement;
use leptos::*;
use markdown::mdast::Node;

pub fn render_md_node(node: &Node) -> HtmlElement<AnyElement> {
    match node {
        Node::Root(root) => view! {
            <div>
                {root.children
                    .iter()
                    .map(|child_node| render_md_node(child_node))
                    .collect::<Vec<_>>()}
            </div>
        }
        .into_any(),
        Node::Paragraph(paragraph) => view! {
            <p>
                {paragraph.children
                    .iter()
                    .map(|child_node| render_md_node(child_node))
                    .collect::<Vec<_>>()}
            </p>
        }
        .into_any(),
        Node::Text(text) => view! { <span>{&text.value}</span> }.into_any(),
        Node::List(list) => {
            if list.ordered {
                view! { <span>"ordered lists are not supported yet"</span> }.into_any()
            } else {
                view! {
                    <ul>
                        {list.children
                            .iter()
                            .map(|list_item| render_md_node(list_item))
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
                    .map(|node| render_md_node(node))
                    .collect_view()}
            </li>
        }
        .into_any(),
        v => view! { <span>{format!("{:?} is not supported", v)}</span> }.into_any(),
    }
}
