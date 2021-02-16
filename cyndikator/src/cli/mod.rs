use structopt::StructOpt;

mod add;
mod init;
mod ls;
mod run;
mod track;

#[derive(StructOpt)]
pub enum Cli {
    Ls(ls::Ls),
    Add(add::Add),
    Run(run::Run),
    Track(track::Track),
    Init(init::Init),
}

impl Cli {
    pub async fn run(self) -> eyre::Result<()> {
        match self {
            Cli::Ls(cmd) => cmd.run().await,
            Cli::Add(cmd) => cmd.run().await,
            Cli::Run(cmd) => cmd.run().await,
            Cli::Track(cmd) => cmd.run().await,
            Cli::Init(cmd) => cmd.run().await,
        }
    }
}
