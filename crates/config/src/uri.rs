use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IgnoreUri {
    pub path: String,
    pub method: Vec<String>,
}
