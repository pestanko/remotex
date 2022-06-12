use std::path::Path;

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    #[serde(rename = "projects")]
    pub project_files: Vec<String>,
    pub web: WebConfig,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            project_files: vec![],
            web: WebConfig::default(),
        }
    }
}

impl AppSettings {
    pub fn load_default_config() -> anyhow::Result<Self> {
        Self::load_config(Path::new("./config"))
    }

    pub fn load_config(root: &Path) -> anyhow::Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::from(root.join("default.yml")))
            .add_source(config::File::from(root.join("local.yml")).required(false))
            .add_source(config::Environment::with_prefix("REMOTEX"))
            .build()?;

        let cfg: AppSettings = settings.try_deserialize()?;

        log::debug!("Loaded config: {:?}", cfg);

        Ok(cfg)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebConfig {
    pub addr: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_real_config() {
        let cfg = AppSettings::load_default_config().unwrap();
        assert!(!cfg.project_files.is_empty())
    }

    #[test]
    fn make_default_settings() {
        let cfg: AppSettings = AppSettings::default();
        assert!(cfg.project_files.is_empty())
    }
}
impl Default for WebConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8030".to_string(),
        }
    }
}
