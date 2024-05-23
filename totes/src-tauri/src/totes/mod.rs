use common::TOTES_PLUGIN_NAME;
use polodb_core::Database;
use tauri::plugin::{Builder, TauriPlugin};
use tauri::{Manager, Runtime};

mod note;
mod space;

const DATABASE_FILEPATH: &str = "../db/totes.db";
const SPACES_COLLECTION_NAME: &str = "spaces";
const NOTES_COLLECTION_NAME: &str = "notes";

pub struct TotesState {
    db: Database,
}

impl TotesState {
    pub fn init() -> Self {
        // TODO(@TheBestTvarynka): implement database filepath detection.
        Self {
            db: Database::open_file(DATABASE_FILEPATH).expect("Database opening should not fail."),
        }
    }
}

pub fn init_totes_plugin<R: Runtime>() -> TauriPlugin<R> {
    Builder::new(TOTES_PLUGIN_NAME)
        .invoke_handler(tauri::generate_handler![
            space::list_spaces,
            space::create_space,
            space::update_space,
            space::delete_space,
            note::list_notes,
            note::create_note,
        ])
        .setup(|app_handle| {
            app_handle.manage(TotesState::init());
            Ok(())
        })
        .build()
}

#[cfg(test)]
mod tests {
    use common::note::Note;
    use common::space::Space;
    use polodb_core::Database;
    use time::macros::datetime;
    use uuid::Uuid;

    use crate::totes::{DATABASE_FILEPATH, NOTES_COLLECTION_NAME, NOTES_COLLECTION_NAME};

    #[test]
    fn seed_database() {
        let db = Database::open_file(DATABASE_FILEPATH).expect("Database opening should not fail.");

        let spaces = db.collection::<Space<'static>>(NOTES_COLLECTION_NAME);
        spaces.drop().expect("Can not drop spaces collection");
        let notes = db.collection::<Note>(NOTES_COLLECTION_NAME);
        notes.drop().expect("Can not drop notes collection");

        db.create_collection(NOTES_COLLECTION_NAME)
            .expect("spaces collection creation should not fail.");
        db.create_collection(NOTES_COLLECTION_NAME)
            .expect("notes collection creation should not fail.");

        let spaces = db.collection::<Space<'static>>(NOTES_COLLECTION_NAME);
        spaces
            .insert_many(vec![
                Space {
                    id: Uuid::new_v4().into(),
                    name: "Q'Kation".into(),
                    created_at: datetime!(2024-05-14 15:03 UTC).into(),
                },
                Space {
                    id: Uuid::new_v4().into(),
                    name: "Memo".into(),
                    created_at: datetime!(2024-05-14 15:03 UTC).into(),
                },
                Space {
                    id: Uuid::new_v4().into(),
                    name: "2024 log".into(),
                    created_at: datetime!(2024-05-14 15:03 UTC).into(),
                },
                Space {
                    id: Uuid::new_v4().into(),
                    name: "work-log".into(),
                    created_at: datetime!(2024-05-18 12:13 UTC).into(),
                },
            ])
            .expect("spaces insertion should not fail");
    }
}
