use std::path::PathBuf;

use crate::runtime::Runtime;

#[derive(Default)]
pub struct ClientBuilder {
    client: Option<reqwest::Client>,
    runtime: Option<PathBuf>,
    database: Option<PathBuf>,
    migrate: Option<bool>,
}

impl ClientBuilder {
    pub fn client(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn runtime(mut self, runtime: PathBuf) -> Self {
        self.runtime = Some(runtime);
        self
    }

    pub fn runtime_opt(mut self, runtime: Option<PathBuf>) -> Self {
        self.runtime = runtime;
        self
    }

    pub fn database(mut self, database: PathBuf) -> Self {
        self.database = Some(database);
        self
    }

    pub fn database_opt(mut self, database: Option<PathBuf>) -> Self {
        self.database = database;
        self
    }

    pub fn migrate(mut self) -> Self {
        self.migrate = Some(true);
        self
    }

    pub async fn build(self) -> crate::Result<super::Client> {
        let rpath = self
            .runtime
            .or_else(|| {
                let mut dir = dirs::config_dir()?;
                dir.push("cyndikator");
                dir.push("init.lua");
                Some(dir)
            })
            .ok_or(crate::Error::InvalidSetup)?;

        let dpath = self
            .database
            .or_else(|| {
                let mut dir = dirs::data_dir()?;
                dir.push("cyndikator");
                dir.push("db.sqlite");
                Some(dir)
            })
            .ok_or(crate::Error::InvalidSetup)?;

        let runtime = Runtime::new(rpath);
        let client = self.client.unwrap_or_default();
        let conn = rusqlite::Connection::open(dpath).map_err(|_| crate::Error::InvalidSetup)?;

        let client = super::Client {
            client,
            runtime,
            conn,
        };

        if self.migrate.unwrap_or(false) {
            client.migrate().await?;
        }

        Ok(client)
    }
}
