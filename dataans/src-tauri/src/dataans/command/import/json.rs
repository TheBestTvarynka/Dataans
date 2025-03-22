use crate::dataans::db::Db;
use crate::dataans::service::note::NoteService;
use crate::dataans::service::space::SpaceService;
use crate::dataans::DataansError;
use common::export::{Schema, SchemaV1};
use serde_json;
use std::fs::File;
use std::path::Path;

pub async fn import_v1<D: Db>(
    schema_v1: SchemaV1,
    space_service: &SpaceService<D>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    for space_data in schema_v1.data {
        let space = space_data.space;
        let notes = space_data.notes;

        let space_exists = space_service.space_by_id(space.id).await.is_ok();
        if !space_exists {
            space_service.create_space(space).await?;
        }

        for note in notes {
            let note_exists = note_service.note_by_id(note.id).await.is_ok();
            if !note_exists {
                note_service.create_note(note).await?;
            }
        }
    }
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
