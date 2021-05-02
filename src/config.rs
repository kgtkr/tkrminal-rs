use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    pub shell: String,
}

impl Config {
    pub fn load() -> Option<Config> {
        let path = env::var("TKRMINAL_CONFIG_PATH")
            .map(PathBuf::from)
            .ok()
            .or_else(|| {
                let mut path = dirs::home_dir()?;
                path.push(".tkrminal.toml");
                Some(path)
            })?;

        let content = fs::read_to_string(path).ok()?;

        toml::from_str(&content).ok()
    }
}
