use reqwest::blocking::get;
use serde_json::Value;

pub fn get_meme() -> Result<String, String> {
    let url = "https://meme-api.com/gimme";

    let res = get(url);

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("Meme API failed, laugh again later : code - {}", response.status()));
            }

            let meme_text = response.text().unwrap_or_default();
            let meme_json: Value = serde_json::from_str(&meme_text).map_err(|_| "Error parsing JSON")?;

            let post_link = meme_json["postLink"].as_str().ok_or("Post link not found in response")?;

            Ok(post_link.to_string())
        }
        Err(_) => Err("Failed to make request".to_string()),
    }
}
