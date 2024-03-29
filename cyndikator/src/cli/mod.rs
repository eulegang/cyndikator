use structopt::StructOpt;

mod init;
mod run;
mod track;
mod view;

#[derive(StructOpt)]
pub enum Cli {
    Run(run::Run),
    Track(track::Track),
    Tracking(track::Tracking),
    Untrack(track::Untrack),
    Init(init::Init),
    View(view::View),
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Run(cmd) => cmd.run().await,
            Cli::Track(cmd) => cmd.run().await,
            Cli::Tracking(cmd) => cmd.run().await,
            Cli::Untrack(cmd) => cmd.run().await,
            Cli::Init(cmd) => cmd.run().await,
            Cli::View(cmd) => cmd.run().await,
        }
    }
}
