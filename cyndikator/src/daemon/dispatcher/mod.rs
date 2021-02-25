use crate::db::{self as db, Database};
use std::str::FromStr;

mod act;
mod cond;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to parse action: {0}")]
    FailedParseAction(String),
    #[error("failed to parse condition: {0}")]
    FailedParseCondition(String),
    #[error("{0}")]
    Database(#[from] db::Error),
}

pub struct Dispatcher {
    dispatches: Vec<(cond::Condition, act::Action)>,
}

pub struct Event<'a> {
    pub feed: &'a str,
    pub feed_url: &'a str,
    pub feed_categories: &'a [String],

    pub title: Option<&'a str>,
    pub link: Option<&'a str>,
    pub author: Option<&'a str>,
    pub categories: &'a [String],
}

impl Dispatcher {
    pub fn load(db: &mut Database) -> Result<Dispatcher, Error> {
        let mut dispatches = Vec::new();

        for (_, condition, action) in db.actions()? {
            let c = cond::Condition::from_str(&condition)
                .or_else(|_| Err(Error::FailedParseCondition(condition)))?;
            let a = act::Action::from_str(&action)
                .or_else(|_| Err(Error::FailedParseAction(action)))?;

            dispatches.push((c, a));
        }

        Ok(Dispatcher { dispatches })
    }

    pub fn dispatch<'a>(&self, event: &Event<'a>, db: &mut Database) {
        for dispatch in &self.dispatches {
            if dispatch.0.satisfied(event) {
                dispatch.1.act(event, db);
            }
        }
    }
}
