pub mod editor;
mod info;
mod md_node;
mod note;

use common::Note as NoteData;
use leptos::{view, *};
use time::macros::datetime;

use self::editor::Editor;
use self::info::Info;
use self::note::Note;

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
            id: 2.into(),
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

    view! {
        <div class="notes-container">
            <Info />
            <div class="notes-inner">
                <div class="notes">
                    {notes
                        .iter()
                        .cloned()
                        .map(|note_data| view! { <Note note=note_data /> })
                        .collect::<Vec<_>>()
                    }
                </div>
                <Editor />
            </div>
        </div>
    }
}
