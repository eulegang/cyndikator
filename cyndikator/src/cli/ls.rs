use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Ls {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,
}

impl Ls {
    pub async fn run(self) -> eyre::Result<()> {
        todo!()
    }
}
