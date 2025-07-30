use leptos::tachys::dom::document;
use wasm_bindgen::JsCast;

pub fn focus_element(id: impl AsRef<str>) {
    let id = id.as_ref();
    if let Some(element) = document().get_element_by_id(id) {
        let element = element
            .dyn_into::<web_sys::HtmlElement>()
            .expect("Expected HtmlElement");
        let _res = element.focus();
        info!("{_res:?}");
    } else {
        warn!("Element not found (id = '{id}')");
    }
}
