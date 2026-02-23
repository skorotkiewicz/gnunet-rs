use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub config_path: Option<PathBuf>,
    pub peer_identity: Option<String>,
    pub cadet_port: u16,
    pub gns_zone: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            config_path: None,
            peer_identity: None,
            cadet_port: 0,
            gns_zone: None,
        }
    }
}

impl Config {
    pub fn from_file(path: impl Into<PathBuf>) -> Self {
        Self {
            config_path: Some(path.into()),
            ..Default::default()
        }
    }
}
