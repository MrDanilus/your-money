use std::fs;
use tauri::AppHandle;
use crate::{funds::Operation, get_operations_file};

#[tauri::command]
pub fn new_operation(
    app: AppHandle,
    description: String, amount: f64, 
    kind: &str, timestamp: String
) -> Result<(), String>{
    let operations_path = match get_operations_file(app){
        Ok(path) => path,
        Err(err) => return Err(err.to_string())
    };

    let mut operations = match fs::read_to_string(&operations_path){
        Ok(file) => match serde_yaml::from_str::<Vec<Operation>>(&file){
            Ok(operations) => operations,
            Err(err) => return Err(err.to_string())
        },
        Err(err) => return Err(err.to_string())
    };
    let operation = Operation{ 
        id: uuid::Uuid::now_v7().to_string(), 
        kind: match kind{
            "income" => super::OperationKind::Income,
            "expense" => super::OperationKind::Expense,
            _ => return Err("Kind not found".to_string())
        }, 
        timestamp, 
        description, 
        amount
    };
    operations.push(operation);

    let operations = match serde_yaml::to_string(&operations){
        Ok(operations) => operations,
        Err(err) => return Err(err.to_string())
    };
    return match fs::write(&operations_path, operations.as_bytes()){
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string())
    }
}