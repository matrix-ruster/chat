use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppConfig {
    pub server: ServerConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let ret: Result<AppConfig> = match (
            File::open("../app.yaml"),
            File::open("/etc/config/app.yaml"),
            env::var("CONFIG_PATH"),
        ) {
            (Ok(reader), _, _) => Ok(serde_yaml::from_reader(reader)?),
            (_, Ok(reader), _) => Ok(serde_yaml::from_reader(reader)?),
            (_, _, Ok(path)) => Ok(serde_yaml::from_reader(File::open(path)?)?),
            _ => anyhow::bail!("config file not found"),
        };
        Ok(ret?)
    }
}
