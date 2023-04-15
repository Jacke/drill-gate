use uuid::Uuid;
use serde::{ Serialize, Deserialize };
use serde_json::Value;

#[derive(Deserialize)]
pub struct Person {
    name: String,
    age: i32,
    is_cool: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    pub id: String,
    pub entry_type: String,
    pub version: String,
    pub meta: Meta,
    pub data: Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}