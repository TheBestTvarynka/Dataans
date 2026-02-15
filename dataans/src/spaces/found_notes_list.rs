use common::Config;
use common::note::Id as NoteId;
use common::space::OwnedSpace;
use leptos::callback::Callback;
use leptos::ev::keydown;
use leptos::prelude::*;
use leptos_use::{use_document, use_event_listener};

use crate::backend::notes::{search_notes, search_notes_in_space};
use crate::dom::MatchKeyBinding;
use crate::notes::note_preview::NotePreview;
use crate::spaces::Space;

#[component]
pub fn FoundNotesList(
    #[allow(unused_variables)] config: Config,
    #[prop(into)] query: Signal<String>,
    search_in_space: Option<OwnedSpace>,
    spaces_minimized: Signal<bool>,
    #[prop(into)] focus_note: Callback<(NoteId, OwnedSpace), ()>,
) -> impl IntoView {
    let (selected_note, set_selected_note) = signal(None);

    let space = search_in_space.clone();
    let found_notes = LocalResource::new(move || {
        let query = query.get();
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
    });

    let select_next_note = move || {
        if let Some(selected_note_id) = selected_note.get()
            && let Some(notes) = found_notes.get()
        {
            let selected_note_index = notes
                .iter()
                .position(|n| n.id == selected_note_id)
                .expect("selected note should present in found notes");
            set_selected_note.set(Some(
                notes
                    .get(if selected_note_index + 1 == notes.len() {
                        0
                    } else {
                        selected_note_index + 1
                    })
                    .expect("valid note index")
                    .id,
            ));
        }
    };
    let select_prev_note = move || {
        if let Some(selected_note_id) = selected_note.get()
            && let Some(notes) = found_notes.get()
        {
            let selected_note_index = notes
                .iter()
                .position(|n| n.id == selected_note_id)
                .expect("selected note should present in found notes");
            set_selected_note.set(Some(
                notes
                    .get(if selected_note_index == 0 {
                        notes.len() - 1
                    } else {
                        selected_note_index - 1
                    })
                    .expect("valid note index")
                    .id,
            ));
        }
    };

    let key_bindings = config.key_bindings.clone();
    let select_next_list_item = key_bindings.select_next_list_item;
    let select_prev_list_item = key_bindings.select_prev_list_item;

    let _ = use_event_listener(use_document(), keydown, move |ev| {
        if select_next_list_item.matches(&ev) {
            ev.prevent_default();
            select_next_note();
        }

        if select_prev_list_item.matches(&ev) {
            ev.prevent_default();
            select_prev_note();
        }
    });

    let global_config = expect_context::<RwSignal<Config>>();

    view! {
        <div class="spaces-scroll-area">
            {move || if let Some(space) = search_in_space.clone() {
                let base_path = global_config.get().app.base_path.clone();

                if spaces_minimized.get() {
                    view! {
                        <div class="note-search-options">
                            <span class="note-search-label">"in:"</span>
                            <Space space base_path set_selected_space=|_| {} selected=true minimized=spaces_minimized />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class="note-search-options">
                            <span class="note-search-label">"Search notes in:"</span>
                            <Space space base_path set_selected_space=|_| {} selected=true minimized=spaces_minimized />
                        </div>
                    }.into_any()
                }
            } else {
                ().into_any()
            }}
            <Suspense
                fallback=move || view! { <span>"Loading notes..."</span> }
            >
                {move || found_notes.get()
                    .map(|notes| if spaces_minimized.get() {
                        view! {
                            <span class="note-search-label">{format!("{}", notes.len())}</span>
                        }
                    } else {
                        view! {
                            <span class="note-search-label">{format!("Found {} notes:", notes.len())}</span>
                        }
                    })}
                {move || found_notes.get()
                    .map(|notes| notes.into_iter().map(|note| {
                        let is_selected = selected_note.get().map(|id| id == note.id).unwrap_or_default();
                        let note_id = note.id;
                        let space = note.space.clone();
                        let base_path = global_config.get().app.base_path.clone();
                        view! {
                            <NotePreview
                                note
                                minimized=spaces_minimized
                                selected=is_selected
                                base_path
                                set_selected_note=move |id| {
                                    set_selected_note.set(Some(id));
                                    focus_note.run((note_id, space.clone()));
                                }/>
                        }
                    }).collect_view())
                }
            </Suspense>
        </div>
    }
}
