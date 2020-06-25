
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub cursor: Option<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult<T> {
    pub data: Vec<T>,

    pub pagination: Option<Pagination>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>
}
