use std::fs::read_to_string;

use common::Theme;

#[tauri::command]
pub fn theme() -> Theme {
    // TODO(@TheBestTvarynka): proper config file path detection.
    let config_file_path = "../configs/theme_dark.toml";

    let config_data = match read_to_string(config_file_path) {
        Ok(data) => data,
        Err(err) => {
            error!("Can not read theme config file: {:?}. Filepath: `{}`.", err, config_file_path);
            return Default::default();
        }
    };

    toml::from_str(&config_data).unwrap_or_else(|err| {
        error!("Can not paste theme config: {:?}", err);
        Default::default()
    })
}
