use std::fs;
use tauri::AppHandle;

use crate::get_operations_file;
use super::Operation;

#[tauri::command]
pub fn get_all_operations(app: AppHandle) -> Result<Vec<Operation>, String>{
    let operations_path = match get_operations_file(app){
        Ok(path) => path,
        Err(err) => return Err(err.to_string())
    };

    let operations = match fs::read_to_string(operations_path){
        Ok(file) => match serde_yaml::from_str::<Vec<Operation>>(&file){
            Ok(operations) => operations,
            Err(err) => return Err(err.to_string())
        },
        Err(err) => return Err(err.to_string())
    };
    return Ok(operations);
}