use std::fs::{self, File};
use std::io::{Error as IoError, Write};

use common::note::{File as NoteFile, Note};
use common::space::{Id as SpaceId, OwnedSpace};
use common::{DataExportConfig, NotesExportOption};
use polodb_core::bson::doc;
use polodb_core::Collection;
use tauri::State;
use time::macros::format_description;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::{DataansState, NOTES_COLLECTION_NAME, SPACES_COLLECTION_NAME};
use crate::BACKUPS_DIR;

fn write_space_notes(
    space_id: SpaceId,
    collection: &Collection<Note<'static>>,
    file: &mut File,
) -> Result<(), IoError> {
    for note in collection
        .find(doc! {
            "space_id": space_id.inner().to_string(),
        })
        .expect("Space notes querying should not fail")
    {
        let Note {
            id,
            text,
            created_at: _,
            space_id,
            files,
        } = note.unwrap();

        write!(file, "### `{}`\n", id.inner().to_string())?;
        write!(file, "Space Id: `{}`\n", space_id.inner().to_string())?;
        // TODO: format creation datetime.
        write!(file, "{}\n", text.as_ref())?;

        write!(file, "#### Files\n")?;
        for note_file in files {
            let NoteFile { id, name, path } = note_file;

            write!(file, "##### {}\n", name)?;
            write!(file, "Id: {}\n", id.to_string())?;
            write!(file, "Path: {:?}\n", path)?;
        }

        write!(file, "---\n")?;
    }

    Ok(())
}

fn write_space(space: &OwnedSpace, file: &mut File) -> Result<(), IoError> {
    let OwnedSpace {
        id,
        name,
        created_at: _,
        avatar,
    } = space;
    write!(file, "# {} \n", name.as_ref())?;

    write!(file, "Id: `{}` \n", id.inner().to_string())?;
    // TODO: format creation datetime.
    write!(file, "Avatar path: `{}` \n", avatar.as_ref())?;

    write!(file, "## {} notes \n", space.name.as_ref())?;

    Ok(())
}

#[instrument(level = "trace", ret, skip(state))]
#[tauri::command]
pub fn export_app_data(state: State<'_, DataansState>, options: DataExportConfig) -> Result<(), String> {
    let backups_dir = state.app_data_dir.join(BACKUPS_DIR);

    if !backups_dir.exists() {
        match fs::create_dir(&backups_dir) {
            Ok(()) => info!(?backups_dir, "Successfully created backups directory"),
            Err(err) => error!(?err, ?backups_dir, "Filed to create backups directory"),
        }
    }

    let format = format_description!("[year].[month].[day]-[hour]:[minute]:[second]");
    let backups_dir = backups_dir.join(
        OffsetDateTime::now_utc()
            .format(&format)
            .map_err(|err| format!("Cannot format datetime: {:?}", err))?,
    );

    fs::create_dir(&backups_dir)
        .map_err(|err| format!("Cannot create backups dir: {:?}. dir: {:?}", err, backups_dir))?;

    match options.notes_export_option {
        NotesExportOption::OneFile => {
            let backup_file = backups_dir.join(format!("dataans-backup-{}.md", Uuid::new_v4()));
            let mut backup_file = File::create(&backup_file)
                .map_err(|err| format!("Cannot create backup file: {:?}. File: {:?}", err, backup_file))?;

            let spaces_collection = state.db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);
            let notes_collection = state.db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

            for space in spaces_collection.find(None).expect("Spaces querying should not fail.") {
                let space = space.unwrap();

                write_space(&space, &mut backup_file).map_err(|err| format!("Cannot write space: {:?}", err))?;
                write_space_notes(space.id, &notes_collection, &mut backup_file)
                    .map_err(|err| format!("Cannot write space notes: {:?}", err))?;
            }
        }
        NotesExportOption::FilePerSpace => {
            todo!()
        }
        NotesExportOption::FilePerNote => {
            todo!()
        }
    }

    Ok(())
}
