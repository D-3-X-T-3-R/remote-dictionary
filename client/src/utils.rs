use crate::data_structures::*;
use rand::seq::IteratorRandom;
use std::collections::HashMap;

pub static SET_MEANING_URL: &str = "http://localhost:8000/set_meaning";
pub static GET_MEANING_URL: &str = "http://localhost:8000/get_meaning";
pub static GET_STATS_URL: &str = "http://localhost:8000/get_stats";

impl SetMeaningRequest {
    pub fn build_set_meaning_request(&mut self, word: String, meaning: String) {
        self.set.push(SetMeaning::new(word, meaning));
    }
}

pub fn create_get_stats_request() {}

pub fn generate_random_word(length: usize) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| alphabet.chars().choose(&mut rng).unwrap())
        .collect()
}
