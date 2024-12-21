use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use common::note::{File as NoteFile, Note, OwnedNote};
use common::space::OwnedSpace;
use common::NotesExportOption;
use futures::future::try_join_all;
use time::macros::format_description;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::dataans::db::Db;
use crate::dataans::service::note::NoteService;
use crate::dataans::DataansError;

fn format_time(time: &OffsetDateTime) -> Result<String, DataansError> {
    let format = format_description!("[year].[month].[day]-[hour].[minute].[second]");

    Ok(time.format(&format)?)
}

async fn write_space_notes_per_file<D: Db>(
    space: &OwnedSpace,
    note_service: &NoteService<D>,
    notes_dir: &Path,
) -> Result<(), DataansError> {
    try_join_all(
        note_service
            .space_notes(space.id)
            .await?
            .into_iter()
            .map(|note| async move {
                let Note {
                    id,
                    text,
                    created_at,
                    space_id: _,
                    files,
                } = note;
                let OwnedSpace {
                    id: space_id,
                    name,
                    avatar,
                    created_at: space_created_at,
                } = &space;

                let mut file = File::create(notes_dir.join(format!("{}.md", id.inner())))?;

                writeln!(file, "# `{}`\n", id.inner())?;

                writeln!(file, "Space Id: `{}`", space_id.inner())?;
                writeln!(file, "Space Name: `{}`", name.as_ref())?;
                writeln!(file, "Space Avatar Id: `{}`", avatar.id())?;
                writeln!(file, "Space Avatar Path: `{}`", avatar.path())?;
                writeln!(file, "Space Created at: `{}`", format_time(space_created_at.as_ref())?)?;

                writeln!(file, "Created at: {}\n", format_time(created_at.as_ref())?)?;
                writeln!(file, "{}\n", text.as_ref())?;

                writeln!(file, "## Files\n")?;
                for note_file in files {
                    let NoteFile { id, name, path } = note_file;

                    writeln!(file, "### {}\n", name)?;
                    writeln!(file, "Id: {}", id)?;
                    writeln!(file, "Path: {:?}\n", path)?;
                }

                Result::<(), DataansError>::Ok(())
            }),
    )
    .await?;

    Ok(())
}

fn write_space_notes(notes: &[OwnedNote], file: &mut File) -> Result<(), DataansError> {
    for note in notes {
        let Note {
            id,
            text,
            created_at,
            space_id,
            files,
        } = note;

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

fn write_space(space: &OwnedSpace, file: &mut File) -> Result<(), DataansError> {
    let OwnedSpace {
        id,
        name,
        created_at,
        avatar,
    } = space;
    writeln!(file, "# {}\n", name.as_ref())?;

    writeln!(file, "Id: `{}`", id.inner())?;
    writeln!(file, "Created at: {}\n", format_time(created_at.as_ref())?)?;
    writeln!(file, "Avatar id: `{}`\n", avatar.id())?;
    writeln!(file, "Avatar path: `{}`\n", avatar.path())?;

    writeln!(file, "## {} notes\n", space.name.as_ref())?;

    Ok(())
}

pub async fn export<D: Db>(
    notes_export_option: &NotesExportOption,
    backups_dir: &Path,
    spaces: Vec<OwnedSpace>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    match notes_export_option {
        NotesExportOption::OneFile => {
            let backup_file_path = backups_dir.join(format!("dataans-backup-{}.md", Uuid::new_v4()));
            let mut backup_file = File::create(&backup_file_path)?;

            for space in spaces {
                write_space(&space, &mut backup_file)?;
                write_space_notes(&note_service.space_notes(space.id).await?, &mut backup_file)?;
            }
        }
        NotesExportOption::FilePerSpace => {
            try_join_all(spaces.into_iter().map(|space| async move {
                let space_file_path = backups_dir.join(format!("{}-{}.md", space.name.as_ref(), space.id.inner()));
                let mut space_file = File::create(&space_file_path)?;

                write_space(&space, &mut space_file)?;
                write_space_notes(&note_service.space_notes(space.id).await?, &mut space_file)?;

                Result::<(), DataansError>::Ok(())
            }))
            .await?;
        }
        NotesExportOption::FilePerNote => {
            try_join_all(spaces.into_iter().map(|space| async move {
                let space_dir = backups_dir.join(format!("{}.{}", space.name.as_ref(), space.id.inner()));
                fs::create_dir(&space_dir)?;

                write_space_notes_per_file(&space, &note_service, &space_dir).await
            }))
            .await?;
        }
    }

    Ok(())
}
