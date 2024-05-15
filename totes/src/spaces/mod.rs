mod space;
mod tools;

use common::space::Space as SpaceData;
use leptos::*;
use time::macros::datetime;

use self::space::Space;
use self::tools::Tools;

fn get_spaces() -> Vec<SpaceData<'static>> {
    vec![
        SpaceData {
            id: 1.into(),
            name: "Q'Kation".into(),
            created_at: datetime!(2024-05-014 15:03 UTC).into(),
        },
        SpaceData {
            id: 2.into(),
            name: "Memo".into(),
            created_at: datetime!(2024-05-014 15:03 UTC).into(),
        },
        SpaceData {
            id: 3.into(),
            name: "2024 log".into(),
            created_at: datetime!(2024-05-014 15:03 UTC).into(),
        },
    ]
}

#[component]
pub fn Spaces() -> impl IntoView {
    let spaces = get_spaces();

    view! {
        <div class="spaces-container">
            <Tools />
            <div class="spaces">
                {spaces.iter().cloned().map(|space| view! {
                    <Space space={space} />
                }).collect_view()}
            </div>
        </div>
    }
}
