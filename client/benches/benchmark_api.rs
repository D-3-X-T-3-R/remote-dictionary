use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

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
pub struct GetMeaningRequest {
    pub words: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStatsRequest {
    pub words: Option<Vec<String>>,
}

fn benchmark_set_meaning_api(c: &mut Criterion) {
    let client = Client::new();

    let url = "http://localhost:8000/set_meaning";

    let payload = black_box(SetMeaningRequest {
        set: vec![
            SetMeaning {
                word: String::from("afforest"),
                meaning: String::from("establish a forest on previously unforested land"),
            },
            SetMeaning {
                word: String::from("aftermath"),
                meaning: String::from(
                    "the consequences of an event, especially a catastrophic one",
                ),
            },
            SetMeaning {
                word: String::from("blithesome"),
                meaning: String::from("carefree and happy and lighthearted"),
            },
            SetMeaning {
                word: String::from("exultant"),
                meaning: String::from("joyful and proud especially because of triumph or success"),
            },
            SetMeaning {
                word: String::from("whimsical"),
                meaning: String::from("determined by chance or impulse rather than by necessity"),
            },
        ],
    });

    let num_requests = 100;

    c.bench_function("Set Meaning request", |b| {
        b.iter(|| {
            for _ in 0..num_requests {
                let _ = client
                    .get(url)
                    .json(&payload)
                    .send()
                    .expect("Failed to send request");

                assert!(true);
            }
        })
    });
}

fn benchmark_get_meaning_api(c: &mut Criterion) {
    let client = Client::new();

    let url = "http://localhost:8000/get_meaning";

    let payload = black_box(GetMeaningRequest {
        words: vec![
            String::from("afforest"),
            String::from("aftermath"),
            String::from("blithesome"),
            String::from("exultant"),
            String::from("whimsical"),
        ],
    });

    let num_requests = 100;

    c.bench_function("Get Meaning request", |b| {
        b.iter(|| {
            for _ in 0..num_requests {
                let _ = client
                    .get(url)
                    .json(&payload)
                    .send()
                    .expect("Failed to send request");

                assert!(true);
            }
        })
    });
}

fn benchmark_get_stats_none_api(c: &mut Criterion) {
    let client = Client::new();

    let url = "http://localhost:8000/get_stats";

    let payload = black_box(GetStatsRequest { words: None });

    let num_requests = 100;

    c.bench_function("Get Stats request none", |b| {
        b.iter(|| {
            for _ in 0..num_requests {
                let _ = client
                    .get(url)
                    .json(&payload)
                    .send()
                    .expect("Failed to send request");

                assert!(true);
            }
        })
    });
}

fn benchmark_get_stats_some_api(c: &mut Criterion) {
    let client = Client::new();

    let url = "http://localhost:8000/get_stats";

    let payload = black_box(GetStatsRequest {
        words: Some(vec![
            String::from("afforest"),
            String::from("aftermath"),
            String::from("blithesome"),
        ]),
    });

    let num_requests = 100;

    c.bench_function("Get Stats request some", |b| {
        b.iter(|| {
            for _ in 0..num_requests {
                let _ = client
                    .get(url)
                    .json(&payload)
                    .send()
                    .expect("Failed to send request");

                assert!(true);
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_set_meaning_api,
    benchmark_get_meaning_api,
    benchmark_get_stats_none_api,
    benchmark_get_stats_some_api
);
criterion_main!(benches);
