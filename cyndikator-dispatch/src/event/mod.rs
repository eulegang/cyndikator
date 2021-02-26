use derive_builder::Builder;

/// An event modeling a rss items and other such notification systems.
#[derive(Builder)]
pub struct Event {
    /// Title of an event
    pub(crate) title: Option<String>,

    /// Categories the event
    pub(crate) categories: Vec<String>,

    /// Title of the feed
    pub(crate) feed_title: Option<String>,

    /// Categories on the feed
    pub(crate) feed_categories: Vec<String>,
}
