use eyre::ContextCompat;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConfig,
    pub dispatch: DispatchConfig,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConfig {
    #[serde(rename = "type", default = "DatabaseConfig::default_db_type")]
    pub ty: String,

    #[serde(default = "DatabaseConfig::default_path")]
    pub path: Option<PathBuf>,
}

impl DatabaseConfig {
    fn default_db_type() -> String {
        "sqlite3".to_string()
    }

    fn default_path() -> Option<PathBuf> {
        dirs::data_dir().map(|mut p| {
            p.push("cyndikator");
            p.push("cynd.sqlite3");
            p
        })
    }
}

#[derive(Deserialize, Debug)]
pub struct DispatchConfig {
    #[serde(rename = "type", default = "DispatchConfig::default_dispatch_type")]
    pub ty: String,

    #[serde(default = "DispatchConfig::default_path")]
    pub path: Option<PathBuf>,
}

impl DispatchConfig {
    fn default_dispatch_type() -> String {
        "dispatch".to_string()
    }

    fn default_path() -> Option<PathBuf> {
        dirs::config_dir().map(|mut p| {
            p.push("cyndikator");
            p.push("dispatch");
            p
        })
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> eyre::Result<Config> {
        let path = path
            .map(Path::to_path_buf)
            .or_else(default_conf)
            .wrap_err("unable to find default cyndikator config path")?;

        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
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
