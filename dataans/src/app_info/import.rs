use leptos::*;

use crate::backend::import::{import_app_data, select_file};

#[component]
pub fn Import() -> impl IntoView {
    let toaster = leptoaster::expect_toaster();
    let (is_importing, set_is_importing) = create_signal(false);

    let import_data = move |_| {
        if is_importing.get() {
            return;
        }

        let toaster_clone = toaster.clone();
        let set_is_importing_clone = set_is_importing.clone();

        set_is_importing_clone.set(true);
        spawn_local(async move {
            match select_file().await {
                Ok(Some(path)) => match import_app_data(path).await {
                    Ok(_) => toaster_clone.success("Import successful!"),
                    Err(e) => toaster_clone.error(&format!("Import failed: {}", e)),
                },
                Ok(None) => {
                    // User canceled the dialog, do nothing
                }
                Err(e) => {
                    toaster_clone.error(&format!("An error occurred: {}", e));
                }
            }
            set_is_importing_clone.set(false);
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
