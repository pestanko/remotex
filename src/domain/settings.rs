use config::Config;
use serde::{Deserialize, Serialize};

type Err = Box<dyn std::error::Error>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    #[serde(rename = "projects")]
    pub project_files: Vec<String>,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            project_files: vec![],
        }
    }
}

impl AppSettings {
    pub fn load_config() -> Result<Self, Err> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config/default.yml"))
            .add_source(config::File::with_name("config/local.yml").required(false))
            .add_source(config::Environment::with_prefix("REMOTEX"))
            .build()?;

        let cfg: AppSettings = settings.try_deserialize()?;

        log::debug!("Loaded config: {:?}", cfg);

        Ok(cfg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_real_config() {
        let cfg = AppSettings::load_config().unwrap();
        assert!(!cfg.project_files.is_empty())
    }

    #[test]
    fn make_default_settings() {
        let cfg: AppSettings = AppSettings::default();
        assert!(cfg.project_files.is_empty())
    }
}
