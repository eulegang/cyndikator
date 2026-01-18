use async_signal::{Signal, Signals};
use futures::StreamExt;
use tokio::sync::mpsc::Sender;
use tokio_util::sync::CancellationToken;

use crate::client::daemon::Action;

pub(crate) struct CheckSignals {
    pub(crate) send: Sender<Action>,
    pub(crate) token: CancellationToken,
}

impl CheckSignals {
    pub(crate) async fn run(self) {
        let mut signals = Signals::new(&[Signal::Usr1, Signal::Int]).unwrap();

        loop {
            let action = tokio::select! {
                _ = self.token.cancelled() => {
                    break;
                }

                signal = signals.next() => {
                    match signal {
                        Some(Ok(Signal::Usr1)) => Action::Reload,
                        Some(Ok(Signal::Int)) => Action::Quit,
                        _ => continue,
                    }
                }
            };

            if self.send.send(action).await.is_err() {
                break;
            }
        }
    }
}
