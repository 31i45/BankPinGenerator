#![windows_subsystem = "windows"]

use tauri::Manager;

#[tauri::command]
fn close_window(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
fn minimize_window(app: tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.minimize();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![close_window, minimize_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
