use clap::Parser;

use crate::Runner;

mod eval;
mod fetch;
mod run;
mod track;
mod untrack;

#[derive(Parser)]
pub enum Cli {
    Fetch(fetch::Fetch),
    Eval(eval::Eval),
    Track(track::Track),
    Untrack(untrack::Untrack),
    Run(run::Run),
}

impl Runner for Cli {
    async fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Eval(eval) => eval.run().await,
            Cli::Fetch(fetch) => fetch.run().await,
            Cli::Track(track) => track.run().await,
            Cli::Untrack(untrack) => untrack.run().await,
            Cli::Run(run) => run.run().await,
        }
    }
}
