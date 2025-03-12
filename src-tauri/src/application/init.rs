pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            super::handlers::create_controllers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
