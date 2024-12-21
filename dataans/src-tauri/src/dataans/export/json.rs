use std::fs::File;
use std::io::Error as IoError;
use std::path::Path;

use common::export::{Schema, SchemaV1, SchemaVersion, Space};
use common::note::OwnedNote;
use common::space::OwnedSpace;
use futures::future::try_join_all;
use uuid::Uuid;

use crate::dataans::db::Db;
use crate::dataans::service::note::NoteService;
use crate::dataans::DataansError;

pub async fn export_v1<D: Db>(
    backups_dir: &Path,
    spaces: Vec<OwnedSpace>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    let backup_file_path = backups_dir.join(format!("dataans-backup-{}.json", Uuid::new_v4()));
    let backup_file = File::create(&backup_file_path)?;

    let data = Schema::V1(SchemaV1 {
        data: try_join_all(spaces.into_iter().map(|space| async move {
            let space_id = space.id;
            Result::<Space, DataansError>::Ok(Space {
                space,
                notes: note_service.space_notes(space_id).await?,
            })
        }))
        .await?,
    });

    serde_json::to_writer(backup_file, &data)?;

    Ok(())
}

pub async fn export<D: Db>(
    version: SchemaVersion,
    backups_dir: &Path,
    spaces: Vec<OwnedSpace>,
    note_service: &NoteService<D>,
) -> Result<(), DataansError> {
    match version {
        SchemaVersion::V1 => export_v1(backups_dir, spaces, note_service).await,
    }
}
