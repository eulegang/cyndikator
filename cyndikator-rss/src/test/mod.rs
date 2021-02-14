use super::*;
use std::str::FromStr;

const CONTENT: &str = include_str!("rss.xml");

#[test]
pub fn parses() {
    let rss = Rss::from_str(CONTENT).expect("clean from_str");
    let mut channel = rss.channel;

    assert_eq!(&channel.title, "Lobsters");
    assert_eq!(&channel.description, "");
    assert_eq!(25, channel.items.len());

    let item = channel.items.remove(0);

    assert_eq!(item.title.as_deref(), Some("Alternative Shells"));
    assert_eq!(
        item.link.as_deref(),
        Some("https://github.com/oilshell/oil/wiki/Alternative-Shells")
    );
    assert_eq!(
        item.author.as_deref(),
        Some("andyc@users.lobste.rs (andyc)")
    );
    assert_eq!(item.category, vec!["unix".to_string()]);
    assert_eq!(
        item.comments.as_deref(),
        Some("https://lobste.rs/s/bhqflt/alternative_shells")
    )
}
