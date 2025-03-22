use gloo_timers::callback::Timeout;
use leptos::*;

use crate::backend::import::{import_app_data, open_file_dialog};

// Define the import state enum
#[derive(Clone)]
enum ImportState {
    Idle,          // No import in progress, no notification
    Importing,     // Import operation is in progress
    Success,       // Import completed successfully
    Error(String), // Import failed with an error message
}

// Constants for timeout durations in milliseconds
const SUCCESS_TIMEOUT: u32 = 3_000; // 3 seconds for success state
const ERROR_TIMEOUT: u32 = 10_000; // 10 seconds for error state

#[component]
pub fn Import() -> impl IntoView {
    let (import_state, set_import_state) = create_signal(ImportState::Idle);

    let import_data = move |_| {
        set_import_state.set(ImportState::Importing);
        spawn_local(async move {
            match open_file_dialog().await {
                Ok(path) => match import_app_data(path).await {
                    Ok(_) => {
                        set_import_state.set(ImportState::Success);
                        // Reset to Idle after SUCCESS_TIMEOUT
                        Timeout::new(SUCCESS_TIMEOUT, move || {
                            set_import_state.set(ImportState::Idle);
                        })
                        .forget();
                    }
                    Err(e) => {
                        set_import_state.set(ImportState::Error(e.to_string()));
                        // Reset to Idle after ERROR_TIMEOUT
                        Timeout::new(ERROR_TIMEOUT, move || {
                            set_import_state.set(ImportState::Idle);
                        })
                        .forget();
                    }
                },
                Err(e) => {
                    set_import_state.set(ImportState::Error(e.to_string()));
                    // Reset to Idle after ERROR_TIMEOUT
                    Timeout::new(ERROR_TIMEOUT, move || {
                        set_import_state.set(ImportState::Idle);
                    })
                    .forget();
                }
            }
        });
    };

    view! {
        <div style="display: flex; align-items: center; gap: 8px;">
            <button on:click=import_data>"Import"</button>
            {move || match import_state.get() {
                ImportState::Success => view! {
                    <img src="/public/icons/green-checkmark.png" alt="Success" style="width: 20px; height: 20px;" />
                }.into_view(),
                ImportState::Error(msg) => view! {
                    <img src="/public/icons/red-cross.png" alt="Error" title={msg} style="width: 20px; height: 20px;" />
                }.into_view(),
                _ => view! { <span></span> }.into_view(),
            }}
        </div>
    }
}
