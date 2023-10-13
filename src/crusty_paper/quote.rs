use reqwest::blocking::get;
use serde_json::Value;

pub fn get_quote() -> Result<String, String> {
    let res = get("https://zenquotes.io/api/today");

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                return Err(format!("Quotes API failed with response code {}.", response.status()));
            }

            let quotes_text = response.text().unwrap_or_default();
            let quotes_json: Value = serde_json::from_str(&quotes_text).map_err(|_| "Error parsing JSON")?;

            let quote = quotes_json[0]["q"].as_str().ok_or("Quote not found in response")?;

            Ok(quote.to_string())
        }
        Err(_) => Err("Failed to make request".to_string()),
    }
}

