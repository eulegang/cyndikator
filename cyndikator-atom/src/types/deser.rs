use super::Timestamp;
use chrono::{DateTime, Local};
use serde::de::{self, Deserialize, Visitor};
use std::fmt;

struct TimestampVisitor;

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = Timestamp;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("expecting an RFC 2822")
    }

    fn visit_str<E>(self, value: &str) -> Result<Timestamp, E>
    where
        E: de::Error,
    {
        match DateTime::parse_from_rfc3339(value) {
            Err(err) => Err(E::custom(format!(
                "'{}' is not a valid rfc 2822 {}",
                value, err
            ))),

            Ok(dt) => {
                let datetime = dt.with_timezone(&Local);
                Ok(Timestamp { datetime })
            }
        }
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimestampVisitor)
    }
}
