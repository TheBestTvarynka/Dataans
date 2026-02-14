use common::error::{CommandError, CommandResult, CommandResultEmpty};
use common::note::File;
use futures::channel::oneshot;
use tauri::{AppHandle, Runtime, State};
use tauri_plugin_dialog::{DialogExt, FilePath};
use uuid::Uuid;

use crate::dataans::DataansState;

#[instrument(ret, skip(state, data))]
#[tauri::command]
pub async fn upload_file(state: State<'_, DataansState>, id: Uuid, name: String, data: Vec<u8>) -> CommandResult<File> {
    Ok(state.file_service.upload_file(id, name, &data).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn delete_file(state: State<'_, DataansState>, id: Uuid) -> CommandResultEmpty {
    Ok(state.file_service.delete_file(id).await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn gen_random_avatar(state: State<'_, DataansState>) -> CommandResult<File> {
    Ok(state.file_service.gen_random_avatar().await?)
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn pick_avatar<R: Runtime>(app: AppHandle<R>, state: State<'_, DataansState>) -> CommandResult<Option<File>> {
    let (tx, rx) = oneshot::channel();

    tauri::async_runtime::spawn(async move {
        app.dialog()
            .file()
            .add_filter(
                "Avatar",
                &["png", "jpg", "jpeg", "bmp", "gif", "webp", "svg", "tiff", "ico"],
            )
            .pick_file(move |file_path| {
                let result = match file_path {
                    Some(FilePath::Path(p)) => Ok(Some(p)),
                    Some(_) => {
                        let err = CommandError::Dataans("unsupported image type selected".to_string());
                        error!(?err, "Failed to select image");

                        Err(err)
                    }
                    None => Ok(None),
                };
                let _ = tx.send(result);
            });
    });

    match rx.await {
        Ok(image_path) => {
            let Some(image_path) = image_path? else {
                debug!("User cancelled file save dialog");

                return Ok(None);
            };

            Ok(Some(state.file_service.pick_avatar(&image_path).await?))
        }
        Err(e) => {
            let err = CommandError::Dataans(format!("failed to receive image path: {e}"));
            error!(?err, "failed to select image");

            Err(err)
        }
    }
}

#[instrument(ret, skip(state))]
#[tauri::command]
pub async fn handle_clipboard_image(state: State<'_, DataansState>) -> CommandResult<File> {
    Ok(state.file_service.handle_clipboard_image().await?)
}

#[tauri::command]
pub async fn save_file_as<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, DataansState>,
    file: File,
) -> CommandResultEmpty {
    let (tx, rx) = oneshot::channel();

    let file_name = file.name.clone();
    tauri::async_runtime::spawn(async move {
        app.dialog()
            .file()
            .set_file_name(file_name)
            .set_title("Save file")
            .save_file(move |file_path| {
                let result = match file_path {
                    Some(FilePath::Path(p)) => Ok(Some(p)),
                    Some(FilePath::Url(_)) => {
                        let err = CommandError::Dataans("URLs are not supported".to_string());
                        error!(?err, "Failed to select file");

                        Err(err)
                    }
                    None => Ok(None),
                };
                let _ = tx.send(result);
            });
    });

    match rx.await {
        Ok(file_path) => {
            let Some(path) = file_path? else {
                debug!("User cancelled file save dialog");

                return Ok(());
            };

            info!("Saving file to {:?}", path);

            state.file_service.save_file_as(&file, &path).await?;

            Ok(())
        }
        Err(err) => {
            error!(?err, "Failed to select file");

            // According to the documentation:
            // > Error returned from a [`Receiver`] when the corresponding [`Sender`] is dropped.
            // So, it is not an actual error in file selection.

            Ok(())
        }
    }
}
