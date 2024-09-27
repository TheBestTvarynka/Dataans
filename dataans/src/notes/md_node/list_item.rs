use leptos::*;

use super::render_md_node;

#[component]
pub fn ListItem(list_item: markdown::mdast::ListItem) -> impl IntoView {
    match list_item.checked {
        None => view! {
            <li>
                {list_item.children
                    .iter()
                    .map(render_md_node)
                    .collect_view()}
            </li>
        }
        .into_any(),
        Some(true) => {
            let id = crate::utils::gen_id();
            view! {
                <li class="note-list-checkbox">
                    // Sorry, I'm tired of CSS and I don't know how to do it better (so far).
                    <input type="checkbox" id=id.clone() checked style="margin-left: -1.5em;" disabled />
                    <label for=id>
                        {list_item.children
                            .iter()
                            .map(render_md_node)
                            .collect_view()}
                    </label>
                </li>
            }
            .into_any()
        }
        Some(false) => {
            let id = crate::utils::gen_id();
            view! {
                <li class="note-list-checkbox">
                    // Sorry, I'm tired of CSS and I don't know how to do it better (so far).
                    <input type="checkbox" id=id.clone() style="margin-left: -1.5em;" disabled />
                    <label for=id>
                        {list_item.children
                            .iter()
                            .map(render_md_node)
                            .collect_view()}
                    </label>
                </li>
            }
            .into_any()
        }
    }
}
