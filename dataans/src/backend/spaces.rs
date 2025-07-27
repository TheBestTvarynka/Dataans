use common::APP_PLUGIN_NAME;
use common::error::{CommandResult, CommandResultEmpty};
use common::space::{CreateSpace, DeleteSpace, OwnedSpace, UpdateSpace};
use serde::Serialize;

use crate::backend::{EmptyArgs, invoke_command};

pub async fn list_spaces() -> CommandResult<Vec<OwnedSpace>> {
    invoke_command(&format!("plugin:{APP_PLUGIN_NAME}|list_spaces"), &EmptyArgs {}).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSpaceArgs<'name, 'avatar> {
    space_data: CreateSpace<'name, 'avatar>,
}

pub async fn create_space(space_data: CreateSpace<'_, '_>) -> CommandResult<OwnedSpace> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|create_space"),
        &CreateSpaceArgs { space_data },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceArgs<'name> {
    space_data: UpdateSpace<'name>,
}

pub async fn update_space(space_data: UpdateSpace<'_>) -> CommandResult<OwnedSpace> {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|update_space"),
        &UpdateSpaceArgs { space_data },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DeleteSpaceArgs {
    space_data: DeleteSpace,
}

pub async fn delete_space(space_data: DeleteSpace) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{APP_PLUGIN_NAME}|delete_space"),
        &DeleteSpaceArgs { space_data },
    )
    .await
}
