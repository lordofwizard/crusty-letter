use reqwest;
use serde::Deserialize;

const MAX_NUMBER_OF_NEWS: usize = 3;

#[derive(Debug, Deserialize)]
struct NewsArticle {
    title: String,
    pubDate: String,
    description: String,
}

pub fn get_news() -> Result<Vec<NewsArticle>, Box<dyn std::error::Error>> {
    let url = "https://newsdata.io/api/1/news?apikey=pub_14522263f5b506fbc598ba7459459c982de1f&language=en&category=top,technology,science&page=1";
    let res = reqwest::blocking::get(url)?;

    if !res.status().is_success() {
        return Err(format!("No news for you today, news api is having some mid life crisis .").into());
    }

    let news: Vec<NewsArticle> = res.json()?;

    Ok(news.iter().take(MAX_NUMBER_OF_NEWS).cloned().collect())
}
