use std::sync::Arc;

use crate::db::types::Feed;

use super::Client;
use tokio::sync::{
    Mutex, Notify,
    mpsc::{Receiver, Sender},
};
use tokio_util::sync::CancellationToken;

mod feeds;
mod fetch;
mod signals;

pub struct Daemon {
    client: Client,
    send: Sender<Action>,
    recv: Receiver<Action>,
}

enum Action {
    Reload,
    Quit,
    Fetch(Feed),
}

trait AsyncOp: Send + Sized + 'static {
    fn run(self) -> impl std::future::Future<Output = crate::Result<()>> + Send;

    fn spawn(self) {
        tokio::spawn(async move {
            if let Err(e) = self.run().await {
                eprintln!("error in async {}", e);
            }
        });
    }
}

impl Daemon {
    pub(crate) fn new(client: Client) -> Self {
        let (send, recv) = tokio::sync::mpsc::channel::<Action>(16);
        Daemon { client, send, recv }
    }

    pub async fn run(self) -> crate::Result<()> {
        let Daemon {
            ref client,
            send,
            mut recv,
        } = self;

        let feeds = Arc::new(Mutex::new(client.conn.list().await?));
        let token = CancellationToken::default();
        let notify = Arc::new(Notify::default());

        let check_signals = signals::CheckSignals {
            send: send.clone(),
            token: token.clone(),
        };
        tokio::spawn(async move { check_signals.run().await });

        let check_feeds = feeds::CheckFeeds {
            send: send.clone(),
            token: token.clone(),
            notify: notify.clone(),
            feeds: feeds.clone(),
        };
        tokio::spawn(async move { check_feeds.run().await });

        while let Some(action) = recv.recv().await {
            match action {
                Action::Reload => {
                    {
                        let mut f = feeds.lock().await;
                        *f = client.conn.list().await?;
                    }
                    notify.notify_waiters();
                }
                Action::Quit => break,

                Action::Fetch(feed) => {
                    let fetch = fetch::FetchFeed {
                        fetcher: self.client.fetcher.clone(),

                        feed,
                        token: token.clone(),
                    };
                    tokio::spawn(async move { fetch.run().await });
                }
            }
        }

        token.cancel();

        Ok(())
    }
}
