pub mod editor;
mod info;
mod md_node;
mod note;

use common::note::Note as NoteData;
use common::space::Space as SpaceData;
use leptos::{view, *};
use time::macros::datetime;
use uuid::Uuid;

use self::editor::Editor;
use self::info::Info;
use self::note::Note;
use crate::app::GlobalState;

// This code will be replaced with real ones.
fn gen_notes() -> Vec<NoteData<'static>> {
    vec![
        //         NoteData {
        //             id: 1.into(),
        //             text: "write a post about it
        // ```rust
        // pub fn get_or_init<F>(&self, f: F) -> &T
        // where
        //     F: FnOnce() -> T,
        // {
        //     match self.get_or_try_init(|| Ok::<T, !>(f())) {
        //         Ok(val) => val,
        //     }
        // }
        // ```
        // https://doc.rust-lang.org/src/std/sync/once_lock.rs.html#246-253
        // "
        //             .into(),
        //             created_at: datetime!(2024-05-01 12:43 UTC).into(),
        //         },
        NoteData {
            id: Uuid::new_v4().into(),
            text: "# Title
what can be better:
* forget *about* smth;
* existing _smth can_ be `improved too`;
  * read **documentation more** carefully.
  * consider [this](https://tbt.qkation.com/about) this ~~blog~~ article.
* https://tbt.qkation.com/about

---

quote:

> Pheww, it starts looking like some kind of Frankenstein
> by TheBestTvarynka

## Second title

1. First item
2. Second item
3. Third item

### Third title

#### Forth title

another **_list_**:
"
            .into(),
            created_at: datetime!(2024-05-014 15:03 UTC).into(),
        },
    ]
}

#[component]
pub fn Notes() -> impl IntoView {
    let notes = gen_notes();

    let global_state = expect_context::<RwSignal<GlobalState>>();

    let (current_state, _) = create_slice(
        global_state,
        |state| state.selected_space.clone(),
        |state, space| state.selected_space = Some(space),
    );

    let (_, set_spaces) = create_slice(
        global_state,
        |state| state.spaces.clone(),
        |state, spaces: Vec<SpaceData>| {
            if let Some(selected_space) = state.selected_space.as_mut() {
                let selected_space_id = selected_space.id;
                if let Some(updated_space) = spaces.iter().find(|s| s.id == selected_space_id) {
                    *selected_space = updated_space.clone();
                } else {
                    state.selected_space = None;
                }
            }
            state.spaces = spaces;
        },
    );

    view! {
        <div class="notes-container">
            <Show
                when=move || current_state.get().is_some()
                fallback=|| view! { <div /> }
            >
                <Info current_space={current_state.get().unwrap()} set_spaces />
            </Show>
            <div class="notes-inner">
                <div class="notes">
                    {notes
                        .iter()
                        .cloned()
                        .map(|note_data| view! { <Note note=note_data /> })
                        .collect::<Vec<_>>()
                    }
                </div>
                <Show when=move || current_state.get().is_some()>
                    <Editor />
                </Show>
            </div>
        </div>
    }
}
