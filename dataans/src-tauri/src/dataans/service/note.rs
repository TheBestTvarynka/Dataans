use std::sync::Arc;

use common::note::{CreateNoteOwned, File, Id as NoteId, Note, NoteFullOwned, OwnedNote, UpdateNote};
use common::space::Id as SpaceId;
use futures::future::try_join_all;
use time::OffsetDateTime;

use crate::dataans::db::model::{File as FileModel, Note as NoteModel};
use crate::dataans::db::Db;
use crate::dataans::service::space::SpaceService;
use crate::dataans::DataansError;

pub struct NoteService<D> {
    db: Arc<D>,
    space_service: Arc<SpaceService<D>>,
}

impl<D: Db> NoteService<D> {
    pub fn new(db: Arc<D>, space_service: Arc<SpaceService<D>>) -> Self {
        Self { db, space_service }
    }

    async fn map_note_model_to_note(note: NoteModel, db: &D) -> Result<OwnedNote, DataansError> {
        let NoteModel {
            id,
            text,
            space_id,
            created_at,
            updated_at,
            is_synced,
        } = note;

        let files = db
            .note_files(id)
            .await?
            .into_iter()
            .map(|file| {
                let FileModel {
                    id,
                    name,
                    path,
                    is_synced: _,
                } = file;
                File {
                    id,
                    name,
                    path: path.into(),
                }
            })
            .collect();

        Ok(OwnedNote {
            id: id.into(),
            text: text.into(),
            space_id: space_id.into(),
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            files,
            is_synced: is_synced.into(),
        })
    }

    pub async fn space_notes(&self, space_id: SpaceId) -> Result<Vec<OwnedNote>, DataansError> {
        let notes = try_join_all(
            self.db
                .space_notes(space_id.inner())
                .await?
                .into_iter()
                .map(|note| Self::map_note_model_to_note(note, &self.db)),
        )
        .await?;

        Ok(notes)
    }

    pub async fn notes(&self) -> Result<Vec<OwnedNote>, DataansError> {
        let notes = try_join_all(
            self.db
                .notes()
                .await?
                .into_iter()
                .map(|note| Self::map_note_model_to_note(note, &self.db)),
        )
        .await?;

        Ok(notes)
    }

    pub async fn create_note(&self, note: CreateNoteOwned) -> Result<(), DataansError> {
        let CreateNoteOwned {
            id,
            text,
            files,
            space_id,
        } = note;

        let created_at = OffsetDateTime::now_utc();

        self.db
            .create_note(&NoteModel {
                id: id.inner(),
                text: text.into(),
                created_at,
                updated_at: created_at,
                space_id: space_id.inner(),
                is_synced: false,
            })
            .await?;

        self.db
            .set_note_files(id.inner(), &files.into_iter().map(|file| file.id).collect::<Vec<_>>())
            .await?;

        Ok(())
    }

    pub async fn update_note(&self, note: UpdateNote<'_>) -> Result<(), DataansError> {
        let UpdateNote {
            id,
            text,
            files,
            is_synced,
        } = note;

        let NoteModel {
            id,
            text: _,
            created_at,
            updated_at: _,
            space_id,
            is_synced: _,
        } = self.db.note_by_id(id.inner()).await?;

        self.db
            .update_note(&NoteModel {
                id,
                text: text.into(),
                created_at,
                updated_at: OffsetDateTime::now_utc(),
                space_id,
                is_synced: is_synced.into(),
            })
            .await?;

        self.db
            .set_note_files(id, &files.into_iter().map(|file| file.id).collect::<Vec<_>>())
            .await?;

        Ok(())
    }

    pub async fn delete_note(&self, note_id: NoteId) -> Result<(), DataansError> {
        self.db.remove_note(note_id.inner()).await?;

        Ok(())
    }

    pub async fn search_notes_in_space(
        &self,
        query: &str,
        space_id: SpaceId,
    ) -> Result<Vec<NoteFullOwned>, DataansError> {
        try_join_all(
            self.space_notes(space_id)
                .await?
                .into_iter()
                .filter(|note| note.text.as_ref().contains(query))
                .map(|note| async move {
                    let Note {
                        id,
                        text,
                        created_at,
                        updated_at,
                        space_id,
                        files,
                        is_synced,
                    } = note;
                    Result::<NoteFullOwned, DataansError>::Ok(NoteFullOwned {
                        id,
                        text,
                        created_at,
                        updated_at,
                        files,
                        space: self.space_service.space_by_id(space_id).await?,
                        is_synced,
                    })
                }),
        )
        .await
    }

    pub async fn search_notes(&self, query: &str) -> Result<Vec<NoteFullOwned>, DataansError> {
        try_join_all(
            self.notes()
                .await?
                .into_iter()
                .filter(|note| note.text.as_ref().contains(query))
                .map(|note| async move {
                    let Note {
                        id,
                        text,
                        created_at,
                        updated_at,
                        space_id,
                        files,
                        is_synced,
                    } = note;
                    Result::<NoteFullOwned, DataansError>::Ok(NoteFullOwned {
                        id,
                        text,
                        created_at,
                        updated_at,
                        files,
                        space: self.space_service.space_by_id(space_id).await?,
                        is_synced,
                    })
                }),
        )
        .await
    }
}
