use chrono::{DateTime, Local};

/// An event modeling a rss items and other such notification systems.
#[derive(Debug)]
pub struct Event {
    /// Url associated with the event
    pub url: Option<String>,

    /// Title of an event
    pub title: Option<String>,

    /// Categories the event
    pub categories: Vec<String>,

    /// Description
    pub description: Option<String>,

    /// Url where the event was found
    pub feed_url: String,

    /// Title of the feed
    pub feed_title: Option<String>,

    /// Categories on the feed
    pub feed_categories: Vec<String>,

    /// DateTime when the event took place
    pub date: Option<DateTime<Local>>,
}
