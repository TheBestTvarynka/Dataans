// https://github.com/tauri-apps/plugins-workspace/blob/715a0477be8f6f77af0377f4eca2b649554446be/plugins/autostart/api-iife.js

use serde_wasm_bindgen::{from_value, to_value};

use crate::backend::{invoke, EmptyArgs};

pub async fn enable() -> bool {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    invoke("plugin:autostart|enable", args).await;

    is_enabled().await
}

pub async fn disable() -> bool {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    invoke("plugin:autostart|disable", args).await;

    is_enabled().await
}

pub async fn is_enabled() -> bool {
    let args = to_value(&EmptyArgs {}).expect("EmptyArgs serialization to JsValue should not fail.");
    let is_enabled = invoke("plugin:autostart|is_enabled", args).await;
    trace!("Is autostart enabled: {:?}.", is_enabled);

    from_value(is_enabled).expect("bool deserialization from JsValue should not fail.")
}
