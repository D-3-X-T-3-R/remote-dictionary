use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct GetStatsRequest {
    #[derivative(Default)]
    pub words: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStatsResponse {
    pub stats: HashMap<String, WordStat>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Derivative)]
#[derivative(Default)]
pub struct WordStat {
    #[derivative(Default)]
    pub total_get_calls: Option<u32>,
    #[derivative(Default)]
    pub success_get_calls: Option<u32>,
    #[derivative(Default)]
    pub failed_get_calls: Option<u32>,
}

impl GetStatsResponse {
    pub fn new() -> Self {
        Self {
            stats: HashMap::new(),
        }
    }
}
