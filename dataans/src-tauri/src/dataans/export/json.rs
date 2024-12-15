use std::fs::File;
use std::io::Error as IoError;
use std::path::Path;

use common::export::{Schema, SchemaV1, SchemaVersion, Space};
use common::note::OwnedNote;
use common::space::OwnedSpace;
use polodb_core::Database;
use uuid::Uuid;

use crate::dataans::note::query_space_notes;
use crate::dataans::space::query_spaces;
use crate::dataans::{NOTES_COLLECTION_NAME, SPACES_COLLECTION_NAME};

pub fn export_v1(backups_dir: &Path, db: &Database) -> Result<(), IoError> {
    let backup_file_path = backups_dir.join(format!("dataans-backup-{}.md", Uuid::new_v4()));
    let backup_file = File::create(&backup_file_path)?;

    let spaces_collection = db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);
    let notes_collection = db.collection::<OwnedNote>(NOTES_COLLECTION_NAME);

    let data = Schema::V1(SchemaV1 {
        data: query_spaces(&spaces_collection)
            .into_iter()
            .map(|space| {
                let notes = query_space_notes(space.id, &notes_collection);
                Space { space, notes }
            })
            .collect(),
    });

    serde_json::to_writer(backup_file, &data)?;

    Ok(())
}

pub fn export(version: SchemaVersion, backups_dir: &Path, db: &Database) -> Result<(), String> {
    match version {
        SchemaVersion::V1 => export_v1(backups_dir, db).map_err(|err| format!("Cannot export data: {:?}", err))?,
    }

    Ok(())
}
