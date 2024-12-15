// TODO:
// Currently, all code is synchronous. But it can be optimized by using async.

use std::fs::{self, File};
use std::io::{Error as IoError, Write};
use std::path::Path;

use common::note::{File as NoteFile, Note};
use common::space::{Id as SpaceId, OwnedSpace};
use common::NotesExportOption;
use polodb_core::bson::doc;
use polodb_core::{Collection, Database};
use time::macros::format_description;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::{NOTES_COLLECTION_NAME, SPACES_COLLECTION_NAME};

fn format_time(time: &OffsetDateTime) -> Result<String, IoError> {
    let format = format_description!("[year].[month].[day]-[hour].[minute].[second]");

    Ok(time.format(&format).expect("OffsetDateTime formatting should not fail"))
}

fn write_space_notes_per_file(
    space_id: SpaceId,
    collection: &Collection<Note<'static>>,
    notes_dir: &Path,
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
            created_at,
            space_id,
            files,
        } = note.unwrap();

        let mut file = File::create(notes_dir.join(format!("{}.md", id.inner())))?;

        writeln!(file, "# `{}`\n", id.inner())?;
        writeln!(file, "Space Id: `{}`", space_id.inner())?;
        writeln!(file, "Created at: {}\n", format_time(created_at.as_ref())?)?;
        writeln!(file, "{}\n", text.as_ref())?;

        writeln!(file, "## Files\n")?;
        for note_file in files {
            let NoteFile { id, name, path } = note_file;

            writeln!(file, "### {}\n", name)?;
            writeln!(file, "Id: {}", id)?;
            writeln!(file, "Path: {:?}\n", path)?;
        }
    }

    Ok(())
}

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
            created_at,
            space_id,
            files,
        } = note.unwrap();

        writeln!(file, "### `{}`\n", id.inner())?;
        writeln!(file, "Space Id: `{}`", space_id.inner())?;
        writeln!(file, "Created at: {}\n", format_time(created_at.as_ref())?)?;
        writeln!(file, "{}\n", text.as_ref())?;

        writeln!(file, "#### Files\n")?;
        for note_file in files {
            let NoteFile { id, name, path } = note_file;

            writeln!(file, "##### {}\n", name)?;
            writeln!(file, "Id: {}", id)?;
            writeln!(file, "Path: {:?}\n", path)?;
        }

        writeln!(file, "---\n")?;
    }

    Ok(())
}

fn write_space(space: &OwnedSpace, file: &mut File) -> Result<(), IoError> {
    let OwnedSpace {
        id,
        name,
        created_at,
        avatar,
    } = space;
    writeln!(file, "# {}\n", name.as_ref())?;

    writeln!(file, "Id: `{}`", id.inner())?;
    writeln!(file, "Created at: {}\n", format_time(created_at.as_ref())?)?;
    writeln!(file, "Avatar path: `{}`\n", avatar.as_ref())?;

    writeln!(file, "## {} notes\n", space.name.as_ref())?;

    Ok(())
}

pub fn export(notes_export_option: &NotesExportOption, backups_dir: &Path, db: &Database) -> Result<(), String> {
    match notes_export_option {
        NotesExportOption::OneFile => {
            let backup_file_path = backups_dir.join(format!("dataans-backup-{}.md", Uuid::new_v4()));
            let mut backup_file = File::create(&backup_file_path)
                .map_err(|err| format!("Cannot create backup file: {:?}. File: {:?}", err, backup_file_path))?;

            let spaces_collection = db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);
            let notes_collection = db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

            for space in spaces_collection.find(None).expect("Spaces querying should not fail.") {
                let space = space.unwrap();

                write_space(&space, &mut backup_file).map_err(|err| format!("Cannot write space: {:?}", err))?;
                write_space_notes(space.id, &notes_collection, &mut backup_file)
                    .map_err(|err| format!("Cannot write space notes: {:?}", err))?;
            }
        }
        NotesExportOption::FilePerSpace => {
            let spaces_collection = db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);
            let notes_collection = db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

            for space in spaces_collection.find(None).expect("Spaces querying should not fail.") {
                let space = space.unwrap();

                let space_file_path = backups_dir.join(format!("{}-{}.md", space.name.as_ref(), space.id.inner()));
                let mut space_file = File::create(&space_file_path)
                    .map_err(|err| format!("Cannot create backup file: {:?}. File: {:?}", err, space_file_path))?;

                write_space(&space, &mut space_file).map_err(|err| format!("Cannot write space: {:?}", err))?;
                write_space_notes(space.id, &notes_collection, &mut space_file)
                    .map_err(|err| format!("Cannot write space notes: {:?}", err))?;
            }
        }
        NotesExportOption::FilePerNote => {
            let spaces_collection = db.collection::<OwnedSpace>(SPACES_COLLECTION_NAME);
            let notes_collection = db.collection::<Note<'static>>(NOTES_COLLECTION_NAME);

            for space in spaces_collection.find(None).expect("Spaces querying should not fail.") {
                let space = space.unwrap();

                let space_dir = backups_dir.join(format!("{}.{}", space.name.as_ref(), space.id.inner()));
                fs::create_dir(&space_dir)
                    .map_err(|err| format!("Cannot create backup dir: {:?}. File: {:?}", err, space_dir))?;

                write_space_notes_per_file(space.id, &notes_collection, &space_dir)
                    .map_err(|err| format!("Cannot write space notes: {:?}", err))?;
            }
        }
    }

    Ok(())
}
