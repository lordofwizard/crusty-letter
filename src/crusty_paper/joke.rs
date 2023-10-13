use reqwest::blocking::get;
use serde_json::Value;

fn get_joke() -> Result<String, String> {
    let res = get("https://v2.jokeapi.dev/joke/Any?blacklistFlags=nsfw,explicit&type=single");

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("Joke API failed with response code {}.", response.status()));
            }

            let joke_text = response.text().unwrap_or_default();
            let joke_json: Value = serde_json::from_str(&joke_text).map_err(|_| "Error parsing JSON")?;
            
            let joke_value = joke_json["joke"].as_str().ok_or("Joke value is not a string")?;

            Ok(joke_value.to_string())
        }
        Err(_) => Err("Failed to make request".to_string()),
    }
}
