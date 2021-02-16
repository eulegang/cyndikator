use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Run {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,
}

impl Run {
    pub async fn run(self) -> eyre::Result<()> {
        todo!()
    }
}
