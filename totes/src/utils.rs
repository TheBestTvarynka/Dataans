use leptos::document;
use wasm_bindgen::JsCast;

pub fn focus_element(id: &str) {
    if let Some(element) = document().get_element_by_id(id) {
        let element = element
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Expected HtmlElement");
        let _res = element.focus();
    } else {
        warn!("Element not found (id = '{id}')");
    }
}
