use leptos::*;

use crate::backend::import::{import_app_data, select_file};

#[component]
pub fn Import() -> impl IntoView {
    let toaster = leptoaster::expect_toaster();
    let (is_importing, set_is_importing) = create_signal(false);

    let import_data = move |_| {
        let toaster_clone = toaster.clone();
        set_is_importing.set(true);

        spawn_local(async move {
            match select_file().await {
                Ok(Some(path)) => match import_app_data(path).await {
                    Ok(_) => toaster_clone.success("Import successful!"),
                    Err(e) => toaster_clone.error(&format!("Import failed: {e}")),
                },
                Ok(None) => {
                    // User canceled the dialog, do nothing
                }
                Err(e) => {
                    toaster_clone.error(&format!("Cannot select a file: {e}"));
                }
            }
            set_is_importing.set(false);
        });
    };

    view! {
        <div>
            <button on:click=import_data disabled=is_importing.get()>
                {move || if is_importing.get() { "Importing..." } else { "Import" }}
            </button>
        </div>
    }
}
