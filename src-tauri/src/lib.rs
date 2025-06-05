mod middlewares;
use middlewares::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_system_tasks, 
            kill_process,
            start_process,
            get_administration_user
            ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
