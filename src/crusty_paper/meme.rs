use reqwest;

fn get_meme() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://meme-api.com/gimme";
    let res = reqwest::get(url);

    if !res.status().is_success() {
        return Err(format!("Meme api failed, laugh again later : code - {}", res.status()).into());
    }

    let result: serde_json::Value = res.json()?;
    if let Some(post_link) = result.get("postLink").and_then(|link| link.as_str()) {
        return Ok(post_link.to_string());
    }

    Err("Meme link not found in response.".into())
}