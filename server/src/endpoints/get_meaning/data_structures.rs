use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMeaningRequest {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMeaningResponse {
    pub word_meanings: HashMap<String, String>,
}

impl GetMeaningResponse {
    pub fn new() -> Self {
        Self {
            word_meanings: HashMap::new(),
        }
    }
}
