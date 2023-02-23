use actix_web::{http::header::ContentType, web, HttpResponse};
use std::sync::Mutex;

use crate::endpoints::{
    data_structures::{RemoteDictionary, WordInfo},
    GetMeaningRequest, GetMeaningResponse,
};

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

    let response = serde_json::to_string(&word_meaning.word_meanings).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
