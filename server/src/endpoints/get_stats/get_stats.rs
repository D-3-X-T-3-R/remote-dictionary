use actix_web::{http::header::ContentType, web, HttpResponse};
use std::sync::Mutex;

use crate::endpoints::{
    data_structures::RemoteDictionary, GetStatsRequest, GetStatsResponse, WordStat,
};

pub async fn get_stats(
    stat_request: web::Json<GetStatsRequest>,
    data: web::Data<Mutex<RemoteDictionary>>,
) -> HttpResponse {
    let mut stat_response = GetStatsResponse::new();
    let remote_dictionary = data.lock().unwrap();
    match &stat_request.words {
        Some(words) => {
            for word in words {
                match remote_dictionary.dict.get(&word.trim().to_lowercase()) {
                    Some(word_info) => {
                        let word_stat = WordStat {
                            total_get_calls: Some(word_info.total_get_calls),
                            success_get_calls: Some(word_info.success_get_calls),
                            failed_get_calls: Some(word_info.failed_get_calls),
                        };
                        stat_response
                            .stats
                            .insert(word.trim().to_lowercase().to_string(), word_stat);
                    }
                    None => {
                        let word_stat = WordStat {
                            total_get_calls: None,
                            success_get_calls: None,
                            failed_get_calls: None,
                        };
                        stat_response
                            .stats
                            .insert(word.trim().to_lowercase().to_string(), word_stat);
                    }
                }
            }
        }
        None => {
            for (key, value) in remote_dictionary.dict.iter() {
                let word_stat = WordStat {
                    total_get_calls: Some(value.total_get_calls),
                    success_get_calls: Some(value.success_get_calls),
                    failed_get_calls: Some(value.failed_get_calls),
                };
                stat_response
                    .stats
                    .insert(key.trim().to_lowercase().to_string(), word_stat);
            }
        }
    }

    let response = serde_json::to_string(&stat_response.stats).unwrap();
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(response)
}
