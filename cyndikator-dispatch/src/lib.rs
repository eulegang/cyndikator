//!
//! A tiny DSL for deciding what to do with an action
//!

use chrono::{DateTime, Local};
use dispatch::Dispatch;
use std::path::Path;

mod dispatch;

pub enum DispatcherSource<'a> {
    Dispatch(&'a Path),
}

impl<'a> DispatcherSource<'a> {
    pub fn dispatcher(self) -> Result<Dispatcher, Error> {
        match self {
            DispatcherSource::Dispatch(path) => {
                let content = std::fs::read_to_string(path)?;

                let dispatch = Dispatch::parse(&content)?;

                Ok(Dispatcher::Dispatch(dispatch))
            }
        }
    }
}

pub enum Dispatcher {
    Dispatch(Dispatch),
}

impl Dispatcher {
    pub fn dispatch(&self, event: &Event) -> Vec<Action> {
        match self {
            Dispatcher::Dispatch(d) => d.dispatch(event),
        }
    }
}

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

/// An action to take given a specific [Event]
#[derive(Debug, PartialEq)]
pub enum Action {
    /// Record the event for viewing later
    Record,

    /// Notify via an approprate channel (system notification system)
    Notify,

    /// Execute a shell line
    Exec(String),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {}", 0)]
    IO(#[from] std::io::Error),

    #[error("dispatch parse error: {}", 0)]
    Parse(#[from] dispatch::ParseError),
}
