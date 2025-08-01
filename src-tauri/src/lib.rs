use std::{fs, path::PathBuf};
use tauri::{AppHandle, Manager};

mod funds;
use funds::{
    get::get_all_operations,
    create::new_operation
};

fn get_operations_file(app: AppHandle) -> Result<PathBuf, String>{
    let operations_path = match app.path().app_local_data_dir(){
        Ok(path) => path.join("operations.yaml"),
        Err(err) => return Err(err.to_string())
    };
    if !operations_path.exists(){
        match fs::write(&operations_path, b""){
            Ok(_) => {},
            Err(err) => return Err(err.to_string())
        }
    }
    Ok(operations_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_all_operations,
            new_operation
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
