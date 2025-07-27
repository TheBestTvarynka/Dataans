use std::fs::File;
use std::path::Path;

use common::event::{DATA_EVENT, DataEvent};
use common::export::{Schema, SchemaV1};
use common::note::{CreateNoteOwned, Note, UpdateNote};
use common::space::{CreateSpaceOwned, Space, UpdateSpace};
use futures::future::try_join_all;
use serde_json;
use tauri::{Emitter, Runtime};

use crate::dataans::db::Db;
use crate::dataans::service::note::{NoteService, NoteServiceError};
use crate::dataans::service::space::{SpaceService, SpaceServiceError};
use crate::dataans::{DataansError, FileService};

fn emit_data_event<R: Runtime, E: Emitter<R>>(emitter: &E, event: DataEvent) -> Result<(), DataansError> {
    emitter.emit(DATA_EVENT, event)?;

    Ok(())
}

pub async fn import_v1<D: Db, R: Runtime, E: Emitter<R>>(
    schema_v1: SchemaV1,
    emitter: &E,
    file_service: &FileService<D>,
    space_service: &SpaceService<D>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    let space_futures = schema_v1.data.into_iter().map(|space_data| async move {
        let space = space_data.space;
        let notes = space_data.notes;

        let Space {
            id,
            name,
            created_at: _,
            updated_at,
            avatar,
        } = space;

        match space_service.space_by_id(space.id).await {
            Err(SpaceServiceError::NotFound) => {
                space_service
                    .create_space(CreateSpaceOwned { id, name, avatar })
                    .await?;

                emit_data_event(
                    emitter,
                    DataEvent::SpaceAdded(space_service.space_by_id(space.id).await?),
                )?;
            }
            Ok(space) => {
                if space.updated_at < updated_at {
                    space_service.update_space(UpdateSpace { id, name, avatar }).await?;

                    emit_data_event(
                        emitter,
                        DataEvent::SpaceAdded(space_service.space_by_id(space.id).await?),
                    )?;
                }
            }
            Err(err) => return Err(DataansError::from(err)),
        }

        let note_futures = notes.into_iter().map(|note| async move {
            let Note {
                id,
                text,
                created_at: _,
                updated_at,
                files,
                space_id,
            } = note;

            let files_futures = files.clone().into_iter().map(|file| async move {
                if file_service.file_by_id(file.id).await.is_err() {
                    let file_id = file.id;
                    file_service.register_file(file).await?;

                    emit_data_event(emitter, DataEvent::FileAdded(file_service.file_by_id(file_id).await?))?;
                }

                Ok::<(), DataansError>(())
            });

            try_join_all(files_futures).await?;

            match note_service.note_by_id(note.id).await {
                Err(NoteServiceError::NotFound) => {
                    note_service
                        .create_note(CreateNoteOwned {
                            id,
                            text,
                            space_id,
                            files,
                        })
                        .await?;

                    emit_data_event(emitter, DataEvent::NoteAdded(note_service.note_by_id(note.id).await?))?;
                }
                Ok(note) => {
                    if note.updated_at < updated_at {
                        note_service.update_note(UpdateNote { id, text, files }).await?;

                        emit_data_event(emitter, DataEvent::NoteUpdated(note_service.note_by_id(note.id).await?))?;
                    }
                }
                Err(err) => return Err(DataansError::from(err)),
            }

            Ok::<(), DataansError>(())
        });

        try_join_all(note_futures).await?;

        Ok::<(), DataansError>(())
    });

    try_join_all(space_futures).await?;

    Ok(())
}

pub async fn import<D: Db, R: Runtime, E: Emitter<R>>(
    emitter: &E,
    file_path: &Path,
    file_service: &FileService<D>,
    space_service: &SpaceService<D>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    let file = File::open(file_path)?;
    let schema: Schema = serde_json::from_reader(file)?;

    match schema {
        Schema::V1(schema_v1) => import_v1(schema_v1, emitter, file_service, space_service, note_service).await,
    }
}
