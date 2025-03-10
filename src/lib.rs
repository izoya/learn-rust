use serde::{Deserialize, Serialize};


// API Docs: https://github.com/15Dkatz/official_joke_api?tab=readme-ov-file
const API_URL: &str = "https://_official-joke-api.appspot.com/jokes";


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn it_works() {
        assert_eq!(1, 1);
    }
}
