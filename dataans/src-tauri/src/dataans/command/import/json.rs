use std::fs::File;
use std::path::Path;

use common::export::{Schema, SchemaV1};
use futures::future::try_join_all;
use serde_json;

use crate::dataans::db::Db;
use crate::dataans::service::note::NoteService;
use crate::dataans::service::space::SpaceService;
use crate::dataans::DataansError;

pub async fn import_v1<D: Db>(
    schema_v1: SchemaV1,
    space_service: &SpaceService<D>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    let space_futures = schema_v1.data.into_iter().map(|space_data| async move {
        let space = space_data.space;
        let notes = space_data.notes;

        if space_service.space_by_id(space.id).await.is_err() {
            space_service.create_space(space).await?;
        }

        let note_futures = notes.into_iter().map(|note| async move {
            if note_service.note_by_id(note.id).await.is_err() {
                note_service.create_note(note).await?;
            }
            Ok::<(), DataansError>(())
        });

        try_join_all(note_futures).await?;
        Ok::<(), DataansError>(())
    });

    // Wait for all spaces to complete
    try_join_all(space_futures).await?;
    Ok(())
}

pub async fn import<D: Db>(
    file_path: &Path,
    space_service: &SpaceService<D>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    let file = File::open(file_path)?;
    let schema: Schema = serde_json::from_reader(file)?;

    match schema {
        Schema::V1(schema_v1) => import_v1(schema_v1, space_service, note_service).await,
    }
}
