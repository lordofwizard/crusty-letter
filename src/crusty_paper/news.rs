use reqwest::blocking::get;
use serde_json::Value;
use std::fmt;
const MAX_NUMBER_OF_NEWS: usize = 3;

pub fn get_news() -> Result<Vec<NewsArticle>, String> {
    let res = get("https://newsdata.io/api/1/news?apikey=pub_14522263f5b506fbc598ba7459459c982de1f&language=en&category=top,technology,science&page=1");

    match res {
        Ok(response) => {
            if !response.status().is_success() {
                return Err("No news for you today, news API is having some mid life crisis.".to_string());
            }

            let news_text = response.text().unwrap_or_default();
            let news_json: Value = serde_json::from_str(&news_text).map_err(|_| "Error parsing JSON")?;

            let mut result = Vec::new();
            for article in news_json["results"].as_array().ok_or("Results not found in response")?.iter().take(MAX_NUMBER_OF_NEWS) {
                let title = article["title"].as_str().unwrap_or_default().to_string();
                let published_date = article["pubDate"].as_str().unwrap_or_default().to_string();
                let summary = article["description"].as_str().unwrap_or_default().to_string();

                let new_article = NewsArticle { title, published_date, summary };
                result.push(new_article);
            }

            Ok(result)
        }
        Err(_) => Err("Failed to make request".to_string()),
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub title: String,
    pub published_date: String,
    pub summary: String,
}

impl fmt::Display for NewsArticle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\nPublished Date: {}\nSummary: {}", self.title, self.published_date, self.summary)
    }
}
