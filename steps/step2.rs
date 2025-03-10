use std::vec::IntoIter;
use serde::{Deserialize, Serialize};

// 5. Add pub fetched_at to Joke
// 6. See if it works
// 7. See if we can show a simple notification to user when something went wrong

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

    async fn fetch_joke(&self) -> Joke {
        let endpoint = "programming/random";
        let url = format!("{}/{}", self.base_url, endpoint);

        let response = reqwest::get(url).await.unwrap();

        let response_data = response.json::<JokesResponse>().await.unwrap();

        let joke = response_data.inner().into_iter().next().unwrap();

        joke
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        let client = JokesClient::new();
        let joke = client.fetch_joke().await;

        //TODO: If we can't get a joke, we'd like to print a human error message

        println!("{:?}", joke);

        // assert_eq!(joke.category, "programming");
    }
}
