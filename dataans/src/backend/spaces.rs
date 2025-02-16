use common::error::{CommandResult, CommandResultEmpty};
use common::space::{CreateSpace, DeleteSpace, OwnedSpace, UpdateSpace};
use common::APP_PLUGIN_NAME;
use serde::Serialize;

use crate::backend::{invoke_command, EmptyArgs};

pub async fn list_spaces() -> CommandResult<Vec<OwnedSpace>> {
    invoke_command(&format!("plugin:{}|list_spaces", APP_PLUGIN_NAME), &EmptyArgs {}).await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateSpaceArgs<'name, 'avatar> {
    space_data: CreateSpace<'name, 'avatar>,
}

pub async fn create_space(space_data: CreateSpace<'_, '_>) -> CommandResult<OwnedSpace> {
    invoke_command(
        &format!("plugin:{}|create_space", APP_PLUGIN_NAME),
        &CreateSpaceArgs { space_data },
    )
    .await
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateSpaceArgs<'name> {
    space_data: UpdateSpace<'name>,
}

pub async fn update_space(space_data: UpdateSpace<'_>) -> CommandResultEmpty {
    invoke_command(
        &format!("plugin:{}|update_space", APP_PLUGIN_NAME),
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
        &format!("plugin:{}|delete_space", APP_PLUGIN_NAME),
        &DeleteSpaceArgs { space_data },
    )
    .await
}
