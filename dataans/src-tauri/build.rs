fn main() {
    println!("cargo:rerun-if-changed=migrations");

    tauri_build::try_build(tauri_build::Attributes::new().plugin(
        "dataans",
        tauri_build::InlinedPlugin::new().commands(&[
            "list_spaces",
            "create_space",
            "update_space",
            "delete_space",
            "list_notes",
            "create_note",
            "update_note",
            "delete_note",
            "search_notes_in_space",
            "search_notes",
            "export_app_data",
            "import_app_data",
            "upload_file",
            "delete_file",
            "gen_random_avatar",
            "handle_clipboard_image",
            "profile",
            "set_sync_options",
            "full_sync",
            "sign_in",
        ]),
    ))
    .expect("Tauri app build should not fail")
}
