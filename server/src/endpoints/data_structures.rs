use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteDictionary {
    pub dict: HashMap<String, WordInfo>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct WordInfo {
    pub meaning: String,
    pub total_get_calls: u32,
    pub success_get_calls: u32,
    pub failed_get_calls: u32,
}

impl RemoteDictionary {
    pub fn new() -> Self {
        Self {
            dict: HashMap::new(),
        }
    }
}
impl WordInfo {
    pub fn new() -> Self {
        Self {
            meaning: "Could Not Find Meaning".to_string(),
            total_get_calls: 0,
            success_get_calls: 0,
            failed_get_calls: 0,
        }
    }
}
