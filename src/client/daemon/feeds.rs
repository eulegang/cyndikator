use std::sync::Arc;

use chrono::{DateTime, Duration, Utc};
use tokio::{
    sync::{Mutex, Notify, mpsc::Sender},
    time::Instant,
};
use tokio_util::sync::CancellationToken;

use crate::{client::daemon::Action, db::types::Feed};

pub(crate) struct CheckFeeds {
    pub(crate) send: Sender<Action>,
    pub(crate) token: CancellationToken,
    pub(crate) notify: Arc<Notify>,
    pub(crate) feeds: Arc<Mutex<Vec<Feed>>>,
}

impl CheckFeeds {
    pub(crate) async fn run(self) {
        let mut current = self.find_current().await;

        loop {
            tokio::select! {
                _ = self.notify.notified() => {
                    current = self.find_current().await;
                }

                _ = self.token.cancelled() => {
                    break;
                }

                _ = CheckFeeds::wait_for(current.clone().map(|(a, _)| a)) => {
                    if let Some(current) = current.take() {
                        let _ = self.send.send(Action::Fetch(current.1.clone())).await;
                    }
                }
            }
        }
    }

    async fn wait_for(date: Option<DateTime<Utc>>) {
        if let Some(date) = date {
            let dur = date - Utc::now();
            let inst = Instant::now() + dur.to_std().unwrap();

            tokio::time::sleep_until(inst.into()).await
        } else {
            futures::future::pending().await
        }
    }

    async fn find_current(&self) -> Option<(DateTime<Utc>, Feed)> {
        let feeds = self.feeds.lock().await;
        feeds
            .iter()
            .map(|feed| {
                let next = feed.last_fetch + Duration::minutes(feed.ttl.into());

                (next, feed)
            })
            .min_by(|(a, _), (b, _)| a.cmp(b))
            .map(|(date, feed)| (date, feed.clone()))
    }
}
