use rss::{Channel, Item};
use std::error::Error;

fn fetch_feed_items() -> Result<Vec<Item>, Box<dyn Error>> {
    let url = "https://b.hatena.ne.jp/hotentry/it.rss";
    let body = reqwest::blocking::get(url)?.bytes()?;

    let channel = Channel::read_from(&body[..])?;

    Ok(channel.into_items())
}

#[derive(Debug)]
struct FeedItem {
    title: String,
    link: String,
}

fn main() {
    let items = fetch_feed_items().unwrap();
    let feed_items: Vec<FeedItem> = items
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
        })
        .collect();

    for item in feed_items {
        println!("================");
        println!("Title: {}", item.title);
        println!("Link: {}", item.link);
    }
}
