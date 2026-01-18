use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Feed {
    pub url: String,
    pub ttl: u32,
    pub last_fetch: DateTime<Utc>,
    pub tracking: u32,
}
