use common::space::OwnedSpace;
use leptos::*;
use leptos_hotkeys::{use_hotkeys, use_hotkeys_scoped};

use crate::backend::notes::{search_notes, search_notes_in_space};
use crate::notes::note_preview::NotePreview;
use crate::spaces::Space;

#[component]
pub fn FoundNotesList(
    #[prop(into)] query: Signal<String>,
    search_in_space: Option<OwnedSpace>,
    spaces_minimized: Signal<bool>,
) -> impl IntoView {
    let (selected_note, set_selected_note) = create_signal(None);

    let space = search_in_space.clone();
    let found_notes = create_resource(
        move || query.get(),
        move |query| {
            let search_in_space = space.clone();
            async move {
                if query.is_empty() {
                    return vec![];
                }
                match search_in_space {
                    Some(space) => search_notes_in_space(space.id, &query)
                        .await
                        .expect("Notes searching should not fail"),
                    None => search_notes(&query).await.expect("Notes searching should not fail"),
                }
            }
        },
    );

    view! {
        <div class="spaces-scroll-area">
            {if let Some(space) = search_in_space {
                view! {
                    <div class="note-search-options">
                        <span class="note-search-label">"Search notes in:"</span>
                        <Space space set_selected_space=|_| {} selected=true minimized={spaces_minimized} />
                    </div>
                }.into_view()
            } else {
                view! {}.into_view()
            }}
            <Suspense
                fallback=move || view! { <span>"Loading notes..."</span> }
            >
                {move || found_notes.get()
                    .map(|notes| view! {
                        <span class="note-search-label">{format!("Found {} notes:", notes.len())}</span>
                    })}
                {move || found_notes.get()
                    .map(|notes| notes.into_iter().map(|note| {
                        let is_selected = selected_note.get().map(|id| id == note.id).unwrap_or_default();
                        view! {
                            <NotePreview
                                note
                                minimized=spaces_minimized
                                selected=is_selected
                                set_selected_note=move |id| set_selected_note.set(Some(id)) />
                        }
                    }).collect_view())
                }
            </Suspense>
        </div>
    }
}
