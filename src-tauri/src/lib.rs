use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Window control commands
#[tauri::command]
fn minimize_window(window: Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize_window(window: Window) {
    window.maximize().unwrap();
}

#[tauri::command]
fn unmaximize_window(window: Window) {
    window.unmaximize().unwrap();
}

#[tauri::command]
fn close_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command]
fn is_window_maximized(window: Window) -> bool {
    window.is_maximized().unwrap()
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
                .invoke_handler(tauri::generate_handler![
                    greet,
                    minimize_window,
                    maximize_window,
                    unmaximize_window,
                    close_window,
                    is_window_maximized
                ])

        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
