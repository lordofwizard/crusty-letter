mod crusty_paper;
use crusty_paper::joke::get_joke;
use crusty_paper::meme::get_meme;
use crusty_paper::news::get_news;
use crusty_paper::quote::get_quote;
fn main() {
    println!("Joke : {:?}",get_joke().unwrap());
    println!("Meme : {:?}",get_meme().unwrap());
    println!("Quote : {:?}",get_quote().unwrap());
    println!("Hello, world!");

    match get_news() {
        Ok(news) => {
            for article in news {
                println!("Title: {}", article.title);
                println!("Published Date: {}", article.published_date);
                println!("Summary: {}\n", article.summary);
            }
        },
        Err(error) => println!("{}", error),
    }
}


