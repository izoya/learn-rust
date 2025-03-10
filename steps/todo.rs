// TODO
// 1. Init the client using config with Default feature.
// 2. Get API_URL from .env
// 3. Adjust for different joke types and multiple jokes like https://official-joke-api.appspot.com/jokes/:type/<any-number>.
// Hint: can use enum for types or fetch types list from the `/types` endpoint;
// can use .or("random".to_string()) to unwrap a type with default value
// 4. Add library that handles logging (log4rs, simplelog, tracing, etc.)
// 5. Try to use if-let and let-else with Results (not practical in this case, but good as an experiment)
// 6. Implement Display for JokesClientError
// 7. (Optional) Explore anyhow crate