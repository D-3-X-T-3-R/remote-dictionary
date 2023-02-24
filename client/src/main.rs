// use benchmark_test::benchmark;
// use serde::{Deserialize, Serialize};
// use serde_json::Value;
// use std::collections::HashMap;
// mod benchmark_test;

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct GetMeaningRequest {
//     pub words: Vec<String>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct GetMeaningResponse {
//     pub word_meanings: HashMap<String, String>,
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let words_vec: Vec<String> = vec!["q".to_string(), "w".to_string(), "r".to_string()];
//     let request = GetMeaningRequest { words: words_vec };
//     let client = reqwest::Client::new();

//     let resp = client
//         .get("http://localhost:8000/get_meaning")
//         .json(&request)
//         .send()
//         .await?
//         .json::<GetMeaningResponse>()
//         .await?;
//     println!("{:#?}", resp);
//     Ok(())
// }

fn main() {
    println!("hi")
}
