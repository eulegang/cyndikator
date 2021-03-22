use super::Dispatch;
use crate::Action;
use crate::Event;
use chrono::{DateTime, Local, TimeZone};

const TEST_TEXT: &str = "
# we like rust
\"rust\" in categories and feed_title is \"Lobsters\" {
  notify
  record
}

title matches /Crust of Rust/ {
    notify
    record
}

\"osdev\" in categories {
  record
}

";

#[test]
fn parse() {
    Dispatch::parse(TEST_TEXT).unwrap();
}

#[test]
fn dispatch() {
    let dispatch = Dispatch::parse("'rust' in categories { record }").unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("foobar".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from("https://lobste.rs/rss"),
        feed_title: Some("foobar".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Record]);

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("foobar".to_string()),
        categories: vec![],
        feed_url: String::from("https://lobste.rs/rss"),
        feed_title: Some("foobar".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![]);

    let dispatch = Dispatch::parse("title matches /rust/i { notify }").unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Curst of Rust".to_string()),
        categories: vec![],
        feed_url: String::from("https://lobste.rs/rss"),
        feed_title: None,
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Notify]);
}

#[test]
fn action_deduplication() {
    let dispatch = Dispatch::parse("title matches /rust/i { notify record notify }").unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Curst of Rust".to_string()),
        categories: vec![],
        feed_url: String::from("https://lobste.rs/rss"),
        feed_title: None,
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Notify, Action::Record]);

    let dispatch = Dispatch::parse(
        "title matches /rust/i { exec \"echo '${title}'\" exec \"echo '${feed_title}'\" }",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec![],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(
        actions,
        vec![
            Action::Exec("echo 'Crust of Rust: Subtyping and Variance'".to_string()),
            Action::Exec("echo 'Youtube'".to_string())
        ]
    );
}

#[test]
fn multiaction() {
    let dispatch = Dispatch::parse(
        "
title matches /rust/i { 
    record
}

'rust' in categories { 
    notify
}

",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Record, Action::Notify,]);
}

#[test]
fn drop() {
    let dispatch = Dispatch::parse(
        "
title matches /rust/i { 
    record
    drop
}

'rust' in categories { 
    notify
}

",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Record]);
}

#[test]
fn parens() {
    let dispatch = Dispatch::parse(
        "
feed_title is 'Youtube' and (title matches /rust/i or title matches /zig/i) {
    record
    drop
}

'rust' in categories {
    notify
}
    ",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Rust / zig arbitrary title".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Record]);

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("foobar".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Lobsters".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Notify]);

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Using rust and zig to blah".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Lobsters".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Notify]);
}

#[test]
fn null() {
    let dispatch = Dispatch::parse(
        "
title is null { 
    notify
}

",
    )
    .unwrap();

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: Some("Crust of Rust: Subtyping and Variance".to_string()),
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![]);

    let actions = dispatch.dispatch(&Event {
        url: None,
        description: Some(String::new()),
        title: None,
        categories: vec!["rust".to_string()],
        feed_url: String::from(
            "https://www.youtube.com/feeds/videos.xml?channel_id=UC_iD0xppBwwsrM9DegC5cQQ",
        ),
        feed_title: Some("Youtube".to_string()),
        feed_categories: vec![],
        date: sample_date(),
    });

    assert_eq!(actions, vec![Action::Notify]);
}

fn sample_date() -> Option<DateTime<Local>> {
    Some(Local.ymd(2000, 1, 15).and_hms(12, 30, 0))
}
