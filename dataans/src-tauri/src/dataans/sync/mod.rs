use std::sync::Arc;

use thiserror::Error;

use crate::dataans::db::Db;

#[derive(Debug, Error)]
pub enum SyncError {}

pub async fn sync_future<D: Db>(db: Arc<D>) {
    let synchronizer = Synchronizer::new(db);
}

struct Synchronizer<D> {
    db: Arc<D>,
}

impl<D: Db> Synchronizer<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn sync_full(&self) -> Result<(), SyncError> {
        Ok(())
    }

    // pub async fn sync_created(&self) -> Result<(), SyncError> {
    //     Ok(())
    // }
}
