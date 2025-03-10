use std::vec::IntoIter;
use serde::{Deserialize, Serialize};

// 8. Make `fetch_joke()` return a Result
// 9. Remove all unwrap() occurrences (network errors, parsing error, and optional vector item)
// 10. See the output changed in test; print out a message according to the result

const API_URL: &str = "https://official-joke-api.appspot.com/jokes";

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
    pub fetched_at: chrono::DateTime<chrono::Utc>,
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
            // TODO: Please, don't panic 🙏️
            Err(err) => panic!("Failed to get joke: {}", err),
            // Err(err) => {
            //     println!("[LOG][ERROR] Error fetching a joke: {:?}", err);
            //     return Err("Whoops, something went wrong with the API request.".to_string())
            // },
        };

        let response_data = response.json::<JokesResponse>().await;

        let response_data = match response_data {
            Ok(r) => r,
            Err(err) => {
                // TODO: show the whole response text in error log
                println!("[LOG][ERROR] Error parsing response: {:?}", err);

                // Wait! Where's the RETURN statement?
                Err("Sorry, this joke was too bad to even parse.".to_string())?
            }
        };

        let joke = response_data.inner().into_iter().next();

        // Could have left an Option<Joke> here? Why an error?
        let joke = match joke {
            None => {
                Err("All the jokes are ruined, mate. Not a single decent one left to tell...".to_string())?
            }
            Some(j) => j
        };

        Ok(joke)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tell_me_a_joke() {
        let client = JokesClient::new();
        // TODO: we only have an error message and can't reliably tell where the error happened
        let joke_result = client.fetch_joke().await;

        // TODO: capture the joke itself to do some assertions
        match joke_result {
            Ok(j) => {
                println!("[USER ERROR]{:?}", j);
            }
            Err(e) => {
                println!("[USER OUTPUT]{:?}", e);
            }
        };

        // assert_eq!(joke.category, "programming");
    }
}
