use eyre::ContextCompat;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize)]
pub struct Config {
    database: Option<PathBuf>,
    dispatch: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Config {
        let database = dirs::data_dir().map(|mut p| {
            p.push("cyndikator");
            p.push("cynd.sqlite3");
            p
        });

        let dispatch = dirs::config_dir().map(|mut p| {
            p.push("cyndikator");
            p.push("dispatch");
            p
        });

        Config { database, dispatch }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> eyre::Result<Config> {
        let default = Config::default();

        let mut config = if let Some(path) = path {
            Config::load_from_path(path)?
        } else {
            let default_path =
                default_conf().wrap_err("unable to find default cyndikator config path")?;

            Config::load_from_path(default_path.as_path())?
        };

        config.fold(default);

        Ok(config)
    }

    pub fn database_path(&self) -> eyre::Result<&Path> {
        let path = self
            .database
            .as_ref()
            .wrap_err("database has never been specified")?;

        Ok(path.as_path())
    }

    pub fn dispatch_path(&self) -> eyre::Result<&Path> {
        let path = self
            .dispatch
            .as_ref()
            .wrap_err("database has never been specified")?;

        Ok(path.as_path())
    }

    fn load_from_path(path: &Path) -> eyre::Result<Config> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn fold(&mut self, other: Config) {
        self.database = self.database.take().or(other.database);
        self.dispatch = self.dispatch.take().or(other.dispatch);
    }
}

fn default_conf() -> Option<PathBuf> {
    if let Some(mut conf) = dirs::config_dir() {
        conf.push("cyndikator");
        conf.push("cynd.toml");
        Some(conf)
    } else {
        None
    }
}
