use actix_web::{web, HttpRequest, Responder, HttpResponse, http::header::ContentType};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Mutex};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMeaningRequest {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMeaning {
    pub word: String,
    pub meaning: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SetMeaningRequest {
    pub set: Vec<SetMeaning>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetMeaningResponse {
    pub word_meanings: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemoteDictionary {
    pub dict: HashMap<String, WordInfo>,
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct WordInfo {
    pub meaning: String,
    pub get_calls: u32,
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

impl GetMeaningResponse {
    pub fn new() -> Self {
        Self {
            word_meanings: HashMap::new(),
        }
    }
}
impl WordInfo {
    pub fn new() -> Self {
        Self {
            meaning: "Could Not Find Meaning".to_string(),
            get_calls: 0,
            success_get_calls: 0,
            failed_get_calls: 0,
        }
    }
}

pub async fn test(_name: HttpRequest) -> impl Responder {
    "called"
}

pub async fn get_meaning(
    get_request: web::Json<GetMeaningRequest>,
    data: web::Data<Mutex<RemoteDictionary>>,
) -> HttpResponse {
    let mut word_meaning = GetMeaningResponse::new();
    let word_list = &get_request.words;
    let mut remote_dictionary = data.lock().unwrap();
    for word in word_list.iter() {
        match remote_dictionary.dict.get_mut(word) {
            Some(mut word_info) => {
                if word_info.meaning == "Could Not Find Meaning" {
                    word_info.failed_get_calls += 1
                } else {
                    word_info.success_get_calls += 1;
                }
                word_meaning
                    .word_meanings
                    .insert(word.to_string(), word_info.meaning.to_string());
            }
            None => {
                let mut word_info = WordInfo::new();
                word_info.failed_get_calls += 1;

                word_meaning
                    .word_meanings
                    .insert(word.to_string(), word_info.meaning.to_string());
                remote_dictionary.dict.insert(word.to_string(), word_info);
            }
        };
    }

    let response = serde_json::to_string(& word_meaning.word_meanings).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
