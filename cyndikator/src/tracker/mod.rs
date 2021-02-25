use std::cmp::{Ord, Ordering};
use std::collections::BinaryHeap;
use url::Url;

use chrono::{DateTime, Duration, Local};

#[derive(PartialEq, Eq, Debug)]
pub struct Trackee {
    pub ttl: u32,
    pub last: DateTime<Local>,
    pub url: Url,
}

pub struct Tracker {
    prio: BinaryHeap<Trackee>,
}

impl Tracker {
    pub fn track(&mut self, trackee: Trackee) {
        self.prio.push(trackee);
    }

    pub fn expired(&mut self, marker: &DateTime<Local>) -> Vec<Trackee> {
        let mut buf = Vec::new();

        while let Some(t) = self.prio.peek() {
            let target = t.last + Duration::minutes(t.ttl as i64);
            if &target < marker {
                buf.push(self.prio.pop().unwrap())
            }
        }

        buf
    }
}

impl Default for Tracker {
    fn default() -> Tracker {
        let prio = Default::default();
        Tracker { prio }
    }
}

impl Ord for Trackee {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_target = self.last + Duration::minutes(self.ttl as i64);
        let other_target = other.last + Duration::minutes(other.ttl as i64);

        self_target
            .cmp(&other_target)
            .reverse()
            .then_with(|| self.url.cmp(&other.url))
    }
}

impl PartialOrd for Trackee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
