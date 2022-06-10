use std::path::Path;

use serde::{Deserialize, Serialize};

use super::settings::AppSettings;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub codename: String,
    pub name: String,
    #[serde(default = "default_desc")]
    pub desc: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub tasks: Vec<Task>,
    pub auth: Authorization,
}

fn default_desc() -> String {
    "No description provided".to_string()
}

fn default_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum TaskType {
    Command { name: String, args: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub name: String,
    pub value: TaskType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Authorization {
    pub tokens: Vec<Token>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub name: String,
    pub value: String,
}

impl Project {
    pub fn load<P: AsRef<Path> + std::fmt::Debug>(
        pth: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        log::info!("Loading project: {:?}", pth);
        let fd = std::fs::File::open(pth)?;
        let project = serde_yaml::from_reader(fd)?;

        Ok(project)
    }
}

pub fn load_projects<'t>(cfg: &'t AppSettings) -> Vec<Project> {
    cfg.project_files
        .iter()
        .filter_map(|pth| match Project::load(pth) {
            Ok(p) => Some(p),
            Err(_) => {
                log::error!("Unable to load project: {}", pth);
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {

    use crate::domain::settings::AppSettings;

    use super::load_projects;
    use super::Project;
    use super::TaskType;

    #[test]
    fn load_example_hello_project() {
        let proj = Project::load("config/projects/hello.yml").unwrap();

        assert_eq!(proj.codename.as_str(), "hello-example");
        assert_eq!(proj.name.as_str(), "Hello example project");
        assert_eq!(
            proj.desc.as_str(),
            "Example hello project to print hello world"
        );
        assert!(proj.enabled);

        // tasks
        assert_eq!(proj.tasks.len(), 1);
        assert_eq!(
            proj.tasks[0].value,
            TaskType::Command {
                name: "echo".to_string(),
                args: vec!["Hello World!".to_string()]
            }
        );
        // auth
        assert_eq!(proj.auth.tokens.len(), 1);
        assert!(proj.auth.enabled);
    }

    #[test]
    fn load_example_projects() {
        let cfg = AppSettings::load_default_config().unwrap();
        let projects = load_projects(&cfg);
        assert!(!projects.is_empty());
    }
}
