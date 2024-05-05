use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Message {
    pub role: String,
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Alternative {
    pub message: Message,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct CompletionAsyncResponse {
    pub alternatives: Vec<Alternative>,
    pub usage: Value,
    pub model_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct OperationError {
    pub code: i32,
    pub message: String,
    pub details: Option<Value>,
}


#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct Operation {
    pub id: String,
    pub created_at: String,
    pub created_by: String,
    pub modified_at: String,
    pub done: bool,
    pub metadata: Option<Value>,
    pub description: String,
    pub error: Option<OperationError>,
    pub response: Option<Value>,
}