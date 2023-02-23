use actix_web::{http::header::ContentType, web, HttpResponse};
use std::sync::Mutex;

use crate::endpoints::{
    data_structures::{RemoteDictionary, WordInfo},
    SetMeaningRequest, SetMeaningResponse,
};

pub async fn set_meaning(
    set_request: web::Json<SetMeaningRequest>,
    data: web::Data<Mutex<RemoteDictionary>>,
) -> HttpResponse {
    let mut remote_dictionary = data.lock().unwrap();
    let mut word_meaning = SetMeaningResponse::new();
    for set in set_request.set.iter() {
        match remote_dictionary
            .dict
            .get_mut(&set.word.trim().to_lowercase())
        {
            Some(word_info) => {
                word_info.meaning = set.meaning.to_string();
                word_meaning.word_meanings.insert(
                    set.word.trim().to_lowercase().to_string(),
                    word_info.meaning.to_string(),
                );
            }
            None => {
                let mut word_info = WordInfo::new();
                word_info.meaning = set.meaning.to_string();
                word_meaning.word_meanings.insert(
                    set.word.trim().to_lowercase().to_string(),
                    word_info.meaning.to_string(),
                );
                remote_dictionary
                    .dict
                    .insert(set.word.trim().to_lowercase().to_string(), word_info);
            }
        }
    }

    let response = serde_json::to_string(&word_meaning.word_meanings).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
