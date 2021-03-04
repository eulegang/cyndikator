/// An event modeling a rss items and other such notification systems.
pub struct Event {
    /// Url associated with the event
    pub url: Option<String>,

    /// Title of an event
    pub title: Option<String>,

    /// Categories the event
    pub categories: Vec<String>,

    /// Url where the event was found
    pub feed_url: String,

    /// Title of the feed
    pub feed_title: Option<String>,

    /// Categories on the feed
    pub feed_categories: Vec<String>,
}
