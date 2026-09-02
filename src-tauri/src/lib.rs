#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let context: tauri::Context<tauri::Wry> = tauri::generate_context!();
    #[cfg(target_os = "linux")]
    glib::set_prgname(Some(context.config().identifier.as_str()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
