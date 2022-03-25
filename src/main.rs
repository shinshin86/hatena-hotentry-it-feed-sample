use chrono::{DateTime, Duration, FixedOffset};
use rss::{Channel, Item};
use std::cmp::Ordering;
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
    date: DateTime<FixedOffset>,
}

fn main() {
    let items = fetch_feed_items().unwrap();

    let mut feed_items: Vec<FeedItem> = items
        .iter()
        .map(|item| FeedItem {
            title: item.title().unwrap().to_string(),
            link: item.link().unwrap().to_string(),
            date: DateTime::parse_from_rfc3339(
                &item.dublin_core_ext().unwrap().dates()[0].to_string(),
            )
            .unwrap(),
        })
        .collect();

    /*
     * Feed item sort(Display on console)
     * ==============
     * old
     * .
     * .
     * .
     * new
     */
    feed_items.sort_by(|a, b| {
        let duration: Duration = a.date - b.date;
        if duration.num_milliseconds() > 0 {
            Ordering::Greater
        } else if duration.num_milliseconds() == 0 {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    });

    for item in feed_items {
        println!("================");
        println!("Title: {}", item.title);
        println!("Link: {}", item.link);
        println!("Date: {}", item.date);
    }
}
