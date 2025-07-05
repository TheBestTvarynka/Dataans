use common::error::CommandResult;
use common::event::{UserContextEvent, USER_CONTEXT_EVENT};
use common::profile::UserContext;
use tauri::{AppHandle, Emitter, Runtime, State};

use crate::dataans::{DataansError, DataansState};

pub fn emit_user_context<R: Runtime>(app: AppHandle<R>, user_context: UserContext) -> Result<(), DataansError> {
    app.emit(USER_CONTEXT_EVENT, UserContextEvent::SignedIn(user_context))?;

    Ok(())
}

#[tauri::command]
pub fn profile(state: State<'_, DataansState>) -> CommandResult<Option<UserContext>> {
    Ok(state.web_service.user_context()?)
}
