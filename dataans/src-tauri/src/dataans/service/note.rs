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
            checksum: _,
            block_id: _,
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
                    checksum: _,
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

    pub async fn create_note(&self, note: CreateNoteOwned) -> Result<OwnedNote, DataansError> {
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
                text: text.clone().into(),
                created_at,
                updated_at: created_at,
                space_id: space_id.inner(),
                // TODO
                checksum: Vec::new(),
                block_id: None,
            })
            .await?;

        self.db
            .set_note_files(id.inner(), &files.iter().map(|file| file.id).collect::<Vec<_>>())
            .await?;

        Ok(Note {
            id,
            text,
            files,
            created_at: created_at.into(),
            updated_at: created_at.into(),
            space_id,
        })
    }

    pub async fn update_note(&self, note: UpdateNote<'static>) -> Result<OwnedNote, DataansError> {
        let UpdateNote {
            id: note_id,
            text,
            files,
        } = note;

        let NoteModel {
            id,
            text: _,
            created_at,
            updated_at: _,
            space_id,
            checksum: _,
            block_id,
        } = self.db.note_by_id(note_id.inner()).await?;

        let updated_at = OffsetDateTime::now_utc();

        self.db
            .update_note(&NoteModel {
                id,
                text: text.clone().into(),
                created_at,
                updated_at,
                space_id,
                // TODO
                checksum: Vec::new(),
                block_id,
            })
            .await?;

        self.db
            .set_note_files(id, &files.iter().map(|file| file.id).collect::<Vec<_>>())
            .await?;

        Ok(Note {
            id: note_id,
            text,
            files,
            created_at: created_at.into(),
            updated_at: updated_at.into(),
            space_id: space_id.into(),
        })
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
                    } = note;
                    Result::<NoteFullOwned, DataansError>::Ok(NoteFullOwned {
                        id,
                        text,
                        created_at,
                        updated_at,
                        files,
                        space: self.space_service.space_by_id(space_id).await?,
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
                    } = note;
                    Result::<NoteFullOwned, DataansError>::Ok(NoteFullOwned {
                        id,
                        text,
                        created_at,
                        updated_at,
                        files,
                        space: self.space_service.space_by_id(space_id).await?,
                    })
                }),
        )
        .await
    }
}
