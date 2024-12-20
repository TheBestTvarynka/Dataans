use std::sync::Arc;

use common::note::{File, Id as NoteId, Note, NoteFull, NoteFullOwned, OwnedNote, UpdateNote};
use common::space::Id as SpaceId;
use futures::future::try_join_all;

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

    pub async fn space_notes(&self, space_id: SpaceId) -> Result<Vec<OwnedNote>, DataansError> {
        let notes = try_join_all(
            self.db
                .space_notes(space_id.inner())
                .await?
                .into_iter()
                .map(|note| async move {
                    let NoteModel {
                        id,
                        text,
                        space_id,
                        created_at,
                    } = note;

                    let files = self
                        .db
                        .note_files(id)
                        .await?
                        .into_iter()
                        .map(|file| {
                            let FileModel { id, name, path } = file;
                            File {
                                id: id.into(),
                                name: name.into(),
                                path: path.into(),
                            }
                        })
                        .collect();

                    Result::<OwnedNote, DataansError>::Ok(OwnedNote {
                        id: id.into(),
                        text: text.into(),
                        space_id: space_id.into(),
                        created_at: created_at.into(),
                        files,
                    })
                }),
        )
        .await?;

        Ok(notes)
    }

    pub async fn create_note(&self, note: OwnedNote) -> Result<(), DataansError> {
        let OwnedNote {
            id,
            text,
            files,
            created_at,
            space_id,
        } = note;

        self.db
            .create_note(&NoteModel {
                id: id.inner(),
                text: text.into(),
                created_at: created_at.into(),
                space_id: space_id.inner(),
            })
            .await?;

        self.db
            .set_note_files(id.inner(), &files.into_iter().map(|file| file.id).collect::<Vec<_>>())
            .await?;

        Ok(())
    }

    pub async fn update_note(&self, note: UpdateNote<'_>) -> Result<(), DataansError> {
        let UpdateNote { id, text, files } = note;

        let NoteModel {
            id,
            text: _,
            created_at,
            space_id,
        } = self.db.note_by_id(id.inner()).await?;

        self.db
            .update_note(&NoteModel {
                id,
                text: text.into(),
                created_at,
                space_id,
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
        query: String,
        space_id: SpaceId,
    ) -> Result<Vec<NoteFullOwned>, DataansError> {
        Ok(try_join_all(
            self.space_notes(space_id)
                .await?
                .into_iter()
                .filter(|note| note.text.as_ref().contains(&query))
                .map(|note| async move {
                    let Note {
                        id,
                        text,
                        created_at,
                        space_id,
                        files,
                    } = note;
                    Result::<NoteFullOwned, DataansError>::Ok(NoteFullOwned {
                        id,
                        text,
                        created_at,
                        files,
                        space: self.space_service.space_by_id(space_id).await?,
                    })
                }),
        )
        .await?)
    }
}
