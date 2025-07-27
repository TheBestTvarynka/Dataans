use std::sync::Arc;

use sha2::{Digest, Sha256};
use web_api_types::{BlockChecksum, Blocks, Operation};

use crate::Result;
use crate::db::{Operation as OperationModel, OperationsDb};

pub struct Data<D> {
    db: Arc<D>,
}

impl<D: OperationsDb> Data<D> {
    pub fn new(db: Arc<D>) -> Self {
        Self { db }
    }

    pub async fn blocks(&self, items_per_block: usize) -> Result<Blocks> {
        let operations = self.db.operations(0).await?;

        let blocks = operations
            .chunks(items_per_block)
            .map(|operations| {
                let mut hasher = Sha256::new();

                for operation in operations {
                    hasher.update(&operation.checksum);
                }

                BlockChecksum::from(hasher.finalize().to_vec())
            })
            .collect::<Vec<_>>();

        Ok(Blocks::from(blocks))
    }

    pub async fn operations(&self, operations_to_skip: usize) -> Result<Vec<Operation>> {
        let operations = self.db.operations(operations_to_skip).await?;

        Ok(operations
            .into_iter()
            .map(|operation| Operation {
                id: operation.id.into(),
                created_at: operation.created_at.into(),
                data: operation.data.into(),
                checksum: operation.checksum.into(),
            })
            .collect())
    }

    pub async fn add_operations(&self, operations: Vec<Operation>) -> Result<()> {
        let operations_models: Vec<OperationModel> = operations
            .into_iter()
            .map(|operation| OperationModel {
                id: operation.id.into(),
                created_at: operation.created_at.into(),
                data: operation.data.into(),
                checksum: operation.checksum.into(),
            })
            .collect();

        self.db.add_operations(&operations_models).await?;

        Ok(())
    }
}
