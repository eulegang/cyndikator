use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Add {
    #[structopt(short, long, env = "CYNDIKATOR_DATABASE")]
    database: Option<String>,

    feed: String,
}

impl Add {
    pub async fn run(self) -> eyre::Result<()> {
        todo!()
    }
}
