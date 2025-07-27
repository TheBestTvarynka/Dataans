use common::error::{CommandResult, CommandResultEmpty};
use common::event::{USER_CONTEXT_EVENT, UserContextEvent};
use common::profile::{Sync, SyncMode, UserContext, UserProfile};
use phraze::cli::ListChoice;
use phraze::generate_a_passphrase;
use tauri::{AppHandle, Emitter, Manager, Runtime, State};
use url::Url;

use crate::dataans::crypto::{EncryptionKey, derive_encryption_key};
use crate::dataans::sync::client::Client;
use crate::dataans::{DataansError, DataansState};

pub fn emit_user_context<R: Runtime>(app: &AppHandle<R>, user_context: UserContext) -> Result<(), DataansError> {
    app.emit(USER_CONTEXT_EVENT, UserContextEvent::SignedIn(user_context))?;

    Ok(())
}

#[tauri::command]
pub async fn profile(state: State<'_, DataansState>) -> CommandResult<Option<UserContext>> {
    Ok(state.web_service.user_context().await?)
}

#[tauri::command]
pub async fn sign_out<R: Runtime>(app: AppHandle<R>, state: State<'_, DataansState>) -> CommandResultEmpty {
    state.web_service.sign_out().await?;

    app.emit(USER_CONTEXT_EVENT, UserContextEvent::SignedOut)
        .map_err(DataansError::from)?;

    Ok(())
}

#[tauri::command]
pub async fn sign_in<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, DataansState>,
    token: String,
    url: Url,
    password: Option<String>,
    salt: Option<String>,
) -> CommandResultEmpty {
    trace!(?token, "Setting CF token");

    let (secret_key, salt, sync_config) = match (password, salt) {
        (Some(password), Some(salt)) => {
            // The user wants to sign in on a new device.

            let secret_key = derive_encryption_key(password.as_bytes(), salt.as_bytes()).map_err(|err| {
                error!(?err, "Failed to derive encryption key");
                DataansError::from(err)
            })?;
            (
                secret_key.to_vec().into(),
                salt.into(),
                Sync {
                    url: url.into(),
                    mode: SyncMode::Manual,
                },
            )
        }
        (None, None) => {
            // The user wants to re-authenticate (previous session token is expired).

            let UserProfile {
                auth_token: _,
                secret_key,
                sync_config,
                salt,
            } = state.web_service.load_user_profile().await?;
            (secret_key, salt, sync_config)
        }
        (Some(password), None) => {
            // The very first sign-in. Basically, it is a local sign up.

            let list = phraze::fetch_list(ListChoice::Medium);
            let salt = generate_a_passphrase(5, "-", true, list);

            let secret_key = derive_encryption_key(password.as_bytes(), salt.as_bytes()).map_err(|err| {
                error!(?err, "Failed to derive encryption key");
                DataansError::from(err)
            })?;
            (
                secret_key.to_vec().into(),
                salt.into(),
                Sync {
                    url: url.into(),
                    mode: SyncMode::Manual,
                },
            )
        }
        (None, Some(_salt)) => {
            return Err(DataansError::InvalidCredentials("salt present, but password does not").into());
        }
    };
    let auth_token = token.into();

    Client::new(
        sync_config.url.as_ref().clone(),
        *EncryptionKey::from_slice(secret_key.as_ref().as_slice()),
        &auth_token,
    )
    .map_err(|err| {
        error!(?err, "Failed to create sync client");
        DataansError::from(err)
    })?
    .auth_health()
    .await
    .map_err(|err| {
        error!(?err, "Failed to perform auth health check");
        DataansError::from(err)
    })?;

    info!("Auth health check is successful!");

    let profile = UserProfile {
        auth_token,
        salt,
        secret_key,
        sync_config: sync_config.clone(),
    };

    state.web_service.authorize(profile).await?;

    emit_user_context(&app, UserContext { sync_config })?;

    if let Some(window) = app.webview_windows().get(crate::window::CF_WINDOW_TITLE) {
        window.close().map_err(DataansError::from)?;
    } else {
        warn!("CF-Auth windows not found.");
    }

    Ok(())
}
