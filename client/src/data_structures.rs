use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMeaning {
    pub word: String,
    pub meaning: String,
}

impl SetMeaning {
    pub fn new(word: String, meaning: String) -> Self {
        Self { word, meaning }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMeaningRequest {
    pub set: Vec<SetMeaning>,
}

impl SetMeaningRequest {
    pub fn new() -> Self {
        Self { set: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMeaningResponse {
    pub word_meanings: HashMap<String, String>,
}

impl SetMeaningResponse {
    pub fn new() -> Self {
        Self {
            word_meanings: HashMap::new(),
        }
    }
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStatsRequest {
    pub words: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStatsResponse {
    pub stats: HashMap<String, WordStat>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WordStat {
    pub total_get_calls: Option<u32>,
    pub success_get_calls: Option<u32>,
    pub failed_get_calls: Option<u32>,
}

impl GetStatsResponse {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct WordInfo {
    pub meaning: String,
    pub total_get_calls: u32,
    pub success_get_calls: u32,
    pub failed_get_calls: u32,
}
