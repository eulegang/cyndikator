use crate::{Entry, Error, Feed};
use quick_xml::de::from_str;
use std::str::FromStr;

impl FromStr for Feed {
    type Err = Error;

    fn from_str(input: &str) -> Result<Feed, Error> {
        let rss: Feed = from_str(input)?;

        Ok(rss)
    }
}

impl FromStr for Entry {
    type Err = Error;

    fn from_str(input: &str) -> Result<Entry, Error> {
        let rss: Entry = from_str(input)?;

        Ok(rss)
    }
}
