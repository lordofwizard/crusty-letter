extern crate genpdf;
use genpdf::fonts;
use genpdf::{Document, SimplePageDecorator};
use std::fs::File;
use std::io::Write;

mod crusty_paper;
use crusty_paper::joke::get_joke;
use crusty_paper::meme::get_meme;
use crusty_paper::news::get_news;
use crusty_paper::quote::get_quote;

fn main() {
    // Retrieve content (jokes, memes, quotes, news)
    let joke = get_joke().unwrap();
    let meme = get_meme().unwrap();
    let quote = get_quote().unwrap();

    // Load a font from the file system (Make sure to have the font files available)
    let font_family = fonts::from_files("/usr/share/fonts/liberation/", "LiberationSans", None)
        .expect("Failed to load font family");

    // Create a document and set the default font family
    let mut doc = Document::new(font_family);

    // Change the default settings
    doc.set_title("Daily Report");

    // Customize the pages
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    // Add content (jokes, memes, quotes, news) to the document
    doc.push(genpdf::elements::Paragraph::new(&format!("Joke: {}", joke)));
    doc.push(genpdf::elements::Paragraph::new(&format!("Meme: {}", meme)));
    doc.push(genpdf::elements::Paragraph::new(&format!("Quote: {}", quote)));

/*    for article in news {
        doc.push(genpdf::elements::Paragraph::new(&format!("News: {}", article)));
    }*/

    // Render the document and write it to a file
    doc.render_to_file("daily_report.pdf").expect("Failed to write PDF file");
}
