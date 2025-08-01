use serde::{Serialize, Deserialize};

pub mod create;
pub mod get;

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation{
    pub id: String,
    pub kind: OperationKind,
    pub timestamp: String,
    pub description: String,
    pub amount: f64
}
#[derive(Serialize, Deserialize, Debug)]
pub enum OperationKind{
    Income,
    Expense
}