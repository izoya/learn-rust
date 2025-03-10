use std::vec::IntoIter;
use anyhow::{anyhow, bail};
use chrono::Utc;
use serde::{Deserialize, Serialize};

// Let's make it look nicer!
// 15. Explore anyhow a bit
// 16. Create ClientError and update the return type
// 17. See why
// 18. Use map for errors
// 19. Test and see how to improve ClientError adding Display

// API Docs: https://github.com/15Dkatz/official_joke_api?tab=readme-ov-file
const API_URL: &str = "https://_official-joke-api.appspot.com/jokes";


#[derive(Debug)]
pub enum JokesClientError {
    NetworkError(String),
    ParseError(String),
}

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

    // async fn fetch_joke(&self) -> anyhow::Result<Joke> {
    async fn fetch_joke(&self) -> Result<Joke, JokesClientError> {
        let endpoint = "programming/random";
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = reqwest::get(url).await;

        let response = match response {
            Ok(r) => r,
            Err(err) => {
                println!("[LOG][ERROR] Error fetching a joke: {:?}", err);
                return Err("Whoops, something went wrong with the API request.".to_string())

                // // 1. Using anyhow
                // bail!(err)

                // // 2. Convert to custom error type
                // return Err(JokesClientError::NetworkError("Error fetching a joke".to_string()));

            },
        };

        let response_data = response.json::<JokesResponse>().await?;

        // let response_data = response.json::<JokesResponse>().await
        //     .map_err(|e| JokesClientError::ParseError(format!("Error parsing response: {:?}", e)))?;

        let joke_option = response_data.inner().into_iter().next();
        let joke_option = update_joke(joke_option);

        let Some(joke) = joke_option else {
            Err(JokesClientError::NetworkError("Error fetching a joke".to_string()))?
        };

        // ... do something with the joke

        Ok(joke)
    }
}

fn update_joke(value: Option<Joke>) -> Option<Joke> {
    let now = Utc::now();

    let mut value = value?;
    value.fetched_at = Some(now);

    Some(value)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tell_me_a_joke() {
        let client = JokesClient::new();

        let joke_result = client.fetch_joke().await;

        let joke = match joke_result {
            Ok(j) => {
                println!("[USER OUTPUT]{:?}", j);
                Some(j)
            }
            Err(e) => {
                // We can now handle each error separately:
                match e {
                    JokesClientError::NetworkError(_) => println!("[USER ERROR - NETWORK] {:?}", e.to_string()),
                    JokesClientError::ParseError(_) => println!("[USER ERROR - PARSING] {:?}", e),
                }

                None
            }
        };

        assert!(joke.is_some());

        // Same as unwrap() but will panic with some hint;
        // Usually used for safe unwraps or in tests.
        let joke_unwrapped = joke.expect("We already checked with is_some()");

        assert!(joke_unwrapped.fetched_at.is_some());
    }
}


