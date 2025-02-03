use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub selected_skins: Vec<String>,
    pub last_update: u64,
    pub auto_refresh: bool,
}

impl UserConfig {
    pub fn load() -> Result<Self, std::io::Error> {
        let data = fs::read_to_string("config.json")?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        fs::write("config.json", serde_json::to_string(self)?)?;
        Ok(())
    }
}