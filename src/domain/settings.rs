use std::path::{Path, PathBuf};

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AppSettings {
    #[serde(rename = "projects")]
    pub project_names: Vec<String>,
    pub web: WebConfig,
    pub root_dir: Option<PathBuf>,
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

        let mut cfg: AppSettings = settings.try_deserialize()?;
        cfg.root_dir = Some(root.into());

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
    fn load_example_hello_config() {
        let cfg = AppSettings::load_config(Path::new("examples/hello")).unwrap();
        assert!(!cfg.project_names.is_empty())
    }

    #[test]
    fn make_default_settings() {
        let cfg: AppSettings = AppSettings::default();
        assert!(cfg.project_names.is_empty())
    }
}
impl Default for WebConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1:8030".to_string(),
        }
    }
}
