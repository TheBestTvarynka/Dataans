use common::Note as NoteData;
use leptos::*;

#[component]
pub fn Note<'text>(note: NoteData<'text>) -> impl IntoView {
    view! {
        <div class="note">
            "Note"
        </div>
    }
}
