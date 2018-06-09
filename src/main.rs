extern crate reqwest;
extern crate scraper;

use scraper::{ElementRef, Html, Selector};

fn main() {
    hn("https://news.ycombinator.com");
}

fn hn(url: &str) {
    let mut response = reqwest::get(url).unwrap();
    assert!(response.status().is_success());

    let html = response.text().unwrap();
    // parses string of HTML
    let fragment = Html::parse_document(&html);
    // parses CSS selectors
    let story_selector = Selector::parse(".athing").unwrap();
    let rank_selector = Selector::parse(".rank").unwrap();
    let title_selector = Selector::parse(".storylink").unwrap();

    // iterate over elements matching .athing selector
    for story in fragment.select(&story_selector) {
        let rank = get_text_with_selector(story, &rank_selector);
        let title = get_text_with_selector(story, &title_selector);
        let url = story
            .select(&title_selector)
            .next()
            .unwrap()
            .value()
            .attr("href")
            .unwrap();

        println!("\n| {} | {}", rank, title);
        println!("{}\n", url);
    }
}

fn get_text_with_selector<'a>(element: ElementRef<'a>, selector: &Selector) -> &'a str {
    element
        .select(&selector)
        .next()
        .unwrap()
        .text()
        .collect::<Vec<_>>()[0]
}
