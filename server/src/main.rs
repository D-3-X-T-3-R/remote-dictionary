use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use endpoints::{data_structures::RemoteDictionary, get_meaning, get_stats, set_meaning};

mod endpoints;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let remode_dictionary = RemoteDictionary::new();
    let remode_dictionary_mutex = web::Data::new(Mutex::new(remode_dictionary));
    HttpServer::new(move || {
        App::new()
            .app_data(remode_dictionary_mutex.clone())
            .route("/get_meaning", web::get().to(get_meaning))
            .route("/set_meaning", web::put().to(set_meaning))
            .route("/get_stats", web::get().to(get_stats))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
