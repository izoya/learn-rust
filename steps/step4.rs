use std::vec::IntoIter;
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Let's make it look nicer!
// 11. Make `fetched_at` optional. See deserialization works. Update `fetched_at`
// 12. Propagate an option
// 13. Unwrap Joke with if-let / let-else
// 14. Can we use if-let / let-else  with Results?


const API_URL: &str = "https://official-joke-api.appspot.com/jokes";
// https://official-joke-api.appspot.com/jokes/random
// https://official-joke-api.appspot.com/jokes/1
// Docs: https://github.com/15Dkatz/official_joke_api?tab=readme-ov-file

#[derive(Debug, Deserialize)]
pub struct JokesResponse(Vec<Joke>);

impl JokesResponse {
    pub fn inner(self) -> Vec<Joke> {
        self.0
    }
}

#[derive(Debug, Deserialize)]
struct Joke {
    pub r#type: String,
    pub setup: String,
    pub punchline: String,
    pub id: i64,
    pub fetched_at: Option<chrono::DateTime<chrono::Utc>>,
}

struct JokesClient {
    client: reqwest::Client,
    base_url: String,
}

#[allow(dead_code)]
impl JokesClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        let base_url = API_URL.to_string();

        JokesClient { client, base_url }
    }

    async fn fetch_joke(&self) -> Result<Joke, String> {
        let endpoint = "programming/random";
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = reqwest::get(url).await;

        // Match - is the simplest way to handle Result
        let response = match response {
            Ok(r) => r,
            Err(err) => {
                println!("[LOG][ERROR] Error fetching a joke: {:?}", err);
                return Err("Whoops, something went wrong with the API request.".to_string())
            },
        };

        let response_data = response.json::<JokesResponse>().await;

        let response_data = match response_data {
            Ok(r) => r,
            Err(err) => {
                // TODO: show response text in error log
                println!("[LOG][ERROR] Error parsing response: {:?}", err);
                Err("Sorry, this joke was too bad to even parse.".to_string())?
            }
        };

        let joke_option = response_data.inner().into_iter().next();
        let joke_option = update_joke(joke_option);

        // Option 1: if-let
        if let Some(joke) = joke_option {
            // do something with the joke
            return Ok(joke);
        }

        // No need to use return OR '?' as it's the last statement
        Err("All the jokes are ruined, mate. Not a single decent one left to tell...".to_string())


        // Option 2: let-else
        // let Some(joke) = joke_option else {
        //     Err("All the jokes are ruined, mate. Not a single decent one left to tell...".to_string())?
        //     // OR
        //     // return Err("All the jokes are ruined, mate. Not a single decent one left to tell...".to_string())
        // };
        //
        // // ... do something with the joke
        //
        // Ok(joke)
    }
}

fn update_joke(value: Option<Joke>) -> Option<Joke> {
    let now = Utc::now();

    // Option 1
    match value {
        None => None,
        Some(mut v) => {
            v.fetched_at = Some(now);
            Some(v)
        }
    }

    // // Option 2
    // let mut value = value?;
    // value.fetched_at = Some(now);
    //
    // Some(value)

    // // Option 3
    // value.map(|mut j| {
    //     j.fetched_at = Some(now);
    //     j
    // })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tell_me_a_joke() {
        let client = JokesClient::new();
        // TODO: we only have an error message and can't reliably tell where the error happened
        let joke_result = client.fetch_joke().await;

        let joke = match joke_result {
            Ok(j) => {
                println!("[USER OUTPUT]{:?}", j);
                Some(j)
            }
            Err(e) => {
                println!("[USER ERROR]{:?}", e);
                None
            }
        };

        assert!(joke.is_some());

        // same as unwrap() but will panic with some hint;
        // Usually used for safe unwraps or in tests.
        let joke_unwrapped = joke.expect("We already checked with is_some()");

        assert!(joke_unwrapped.fetched_at.is_some());
    }
}
