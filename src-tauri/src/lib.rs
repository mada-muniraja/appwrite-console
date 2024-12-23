// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            // Update the URL to https://cloud.appwrite.io
            window
                .eval("window.location.replace('https://cloud.appwrite.io');")
                .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
