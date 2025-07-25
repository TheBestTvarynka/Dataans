use leptos::prelude::*;

#[component]
pub fn Switch(id: String, state: bool, setter: Callback<bool>) -> impl IntoView {
    view! {
        <span class="switch-span">
            <input type="checkbox" id=id.clone() class="switch-input" on:change=move |_| setter.call(!state) checked=state />
            <label for=id class="switch-label">"Toggle"</label>
        </span>
    }
}
