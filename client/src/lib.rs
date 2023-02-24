use data_structures::*;
use reqwest::{
    blocking::{Client, Response},
    Error,
};
use std::collections::HashMap;
use utils::*;

mod data_structures;
mod utils;


#[cfg(test)]
mod set_meaning_test {
    use super::*;

    #[test]
    fn set_meaning_test_with_value() {
        let mut set_meaning_request = SetMeaningRequest::new();
        set_meaning_request.build_set_meaning_request(
            String::from("afforest"),
            String::from("establish a forest on previously unforested land"),
        );
        set_meaning_request.build_set_meaning_request(
            String::from("aftermath"),
            String::from("the consequences of an event, especially a catastrophic one"),
        );
        let response = set_meaning(set_meaning_request)
            .expect("Failed to send request")
            .json::<SetMeaningResponse>()
            .unwrap();

        let mut expected_response: HashMap<String, String> = HashMap::new();
        expected_response.insert(
            String::from("afforest"),
            String::from("establish a forest on previously unforested land"),
        );
        expected_response.insert(
            String::from("aftermath"),
            String::from("the consequences of an event, especially a catastrophic one"),
        );

        assert_eq!(response.word_meanings, expected_response);
    }

    #[test]
    fn set_meaning_test_empty() {
        let set_meaning_request = SetMeaningRequest::new();
        let response = set_meaning(set_meaning_request)
            .expect("Failed to send request")
            .json::<SetMeaningResponse>()
            .unwrap();

        let expected_response: HashMap<String, String> = HashMap::new();

        assert_eq!(response.word_meanings, expected_response);
    }

    #[test]
    fn set_meaning_test_upadate_meaning() {
        let mut set_meaning_request = SetMeaningRequest::new();
        set_meaning_request
            .build_set_meaning_request(String::from("afforest"), String::from("Updated meaning"));

        let response = set_meaning(set_meaning_request)
            .expect("Failed to send request")
            .json::<SetMeaningResponse>()
            .unwrap();

        let mut expected_response: HashMap<String, String> = HashMap::new();
        expected_response.insert(String::from("afforest"), String::from("Updated meaning"));

        assert_eq!(response.word_meanings, expected_response);
    }
}

#[cfg(test)]
mod get_meaning_test {
    use super::*;

    #[test]
    fn get_meaning_test_exist() {
        let get_meaning_request = GetMeaningRequest {
            words: vec![String::from("afforest"), String::from("aftermath")],
        };
        let response = get_meaning(get_meaning_request)
            .expect("Failed to send request")
            .json::<GetMeaningResponse>()
            .unwrap();

        let mut expected_response: HashMap<String, String> = HashMap::new();
        expected_response.insert(
            String::from("afforest"),
            String::from("establish a forest on previously unforested land"),
        );
        expected_response.insert(
            String::from("aftermath"),
            String::from("the consequences of an event, especially a catastrophic one"),
        );

        assert_eq!(response.word_meanings, expected_response);
    }

    #[test]
    fn get_meaning_test_does_not_exist() {
        let get_meaning_request = GetMeaningRequest {
            words: vec![String::from("house")],
        };
        let response = get_meaning(get_meaning_request)
            .expect("Failed to send request")
            .json::<GetMeaningResponse>()
            .unwrap();

        let mut expected_response: HashMap<String, String> = HashMap::new();
        expected_response.insert(
            String::from("house"),
            String::from("Could Not Find Meaning"),
        );

        assert_eq!(response.word_meanings, expected_response);
    }
}

#[cfg(test)]
mod get_stats_test {
    use super::*;

    #[test]
    fn get_stats_test_succ_calls() {

        let random_word = generate_random_word(6);
        let mut set_meaning_request = SetMeaningRequest::new();
        set_meaning_request
            .build_set_meaning_request(random_word.to_string(), random_word.to_string());
        let _ = set_meaning(set_meaning_request).unwrap();
        let get_meaning_request = GetMeaningRequest {
            words: vec![random_word.to_string()],
        };

        for _ in 0..50 {
            let _ = get_meaning(get_meaning_request.clone());
        }

        let get_stats_request = GetStatsRequest {
            words: Some(vec![random_word.to_string()]),
        };

        let response = get_stat(get_stats_request)
            .expect("Failed to send request")
            .json::<GetStatsResponse>()
            .unwrap();

        assert_eq!(
            response.stats.get(&random_word).unwrap().success_get_calls,
            Some(50)
        );
    }

    #[test]
    fn get_stats_test_fail_calls() {

        let random_word = generate_random_word(6);

        let get_meaning_request = GetMeaningRequest {
            words: vec![random_word.to_string()],
        };

        for _ in 0..50 {
            let _ = get_meaning(get_meaning_request.clone());
        }

        let get_stats_request = GetStatsRequest {
            words: Some(vec![random_word.to_string()]),
        };

        let response = get_stat(get_stats_request)
            .expect("Failed to send request")
            .json::<GetStatsResponse>()
            .unwrap();

        assert_eq!(
            response.stats.get(&random_word).unwrap().failed_get_calls,
            Some(50)
        );
    }

    #[test]
    fn get_stats_test_total_calls() {

        let random_word = generate_random_word(6);
        let get_meaning_request = GetMeaningRequest {
            words: vec![random_word.to_string()],
        };
        
        for _ in 0..20 {
            let _ = get_meaning(get_meaning_request.clone());
        }

        let mut set_meaning_request = SetMeaningRequest::new();
        set_meaning_request
            .build_set_meaning_request(random_word.to_string(), random_word.to_string());
        let _ = set_meaning(set_meaning_request).unwrap();

        for _ in 0..30 {
            let _ = get_meaning(get_meaning_request.clone());
        }
        let get_stats_request = GetStatsRequest {
            words: Some(vec![random_word.to_string()]),
        };

        let response = get_stat(get_stats_request)
            .expect("Failed to send request")
            .json::<GetStatsResponse>()
            .unwrap();

        assert_eq!(
            response.stats.get(&random_word).unwrap().total_get_calls,
            Some(50)
        );
        assert_eq!(
            response.stats.get(&random_word).unwrap().failed_get_calls,
            Some(20)
        );
        assert_eq!(
            response.stats.get(&random_word).unwrap().success_get_calls,
            Some(30)
        );
    }

    #[test]
    fn get_stats_test_word_does_not_exist() {

        let random_word = generate_random_word(6);

        let get_stats_request = GetStatsRequest {
            words: Some(vec![random_word.to_string()]),
        };

        let response = get_stat(get_stats_request)
            .expect("Failed to send request")
            .json::<GetStatsResponse>()
            .unwrap();

        assert_eq!(
            response.stats.get(&random_word).unwrap().failed_get_calls,
            None
        );
    }
}

fn set_meaning(set_meaning_request: SetMeaningRequest) -> Result<Response, Error> {
    let client = Client::new();

    let response = client
        .put(SET_MEANING_URL)
        .json(&set_meaning_request)
        .send();
    response
}

fn get_meaning(get_meaning_request: GetMeaningRequest) -> Result<Response, Error> {
    let client = Client::new();

    let response = client
        .get(GET_MEANING_URL)
        .json(&get_meaning_request)
        .send();
    response
}

fn get_stat(get_stat_request: GetStatsRequest) -> Result<Response, Error> {
    let client = Client::new();

    let response = client.get(GET_STATS_URL).json(&get_stat_request).send();
    response
}
