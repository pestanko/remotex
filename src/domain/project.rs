use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{auth::Auth, settings::AppSettings};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub codename: String,
    pub name: String,
    #[serde(default = "default_desc")]
    pub desc: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub tasks: Vec<Task>,
    pub auth: Auth,
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

impl Project {
    pub fn load_file<P: AsRef<Path> + std::fmt::Debug>(
        pth: P,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        log::info!("Loading project: {:?}", pth);
        let fd = std::fs::File::open(pth)?;
        let project = serde_yaml::from_reader(fd)?;

        Ok(project)
    }
}

pub fn load_projects(cfg: &AppSettings) -> Vec<Project> {
    let root: &Path = cfg
        .root_dir
        .as_ref()
        .expect("Root directory has to initialized at this point");

    cfg.project_names
        .iter()
        .filter_map(
            |name| match Project::load_file(make_project_path(root, name)) {
                Ok(p) => Some(p),
                Err(_) => {
                    log::error!("Unable to load project: {name}");
                    None
                }
            },
        )
        .collect()
}

fn make_project_path(root: &Path, name: &str) -> PathBuf {
    root.join("projects").join(format!("{name}.yml"))
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use crate::domain::settings::AppSettings;

    use super::load_projects;
    use super::Project;
    use super::TaskType;

    #[test]
    fn load_example_hello_project() {
        let proj = Project::load_file("examples/hello/projects/hello.yml").unwrap();

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
        let cfg = AppSettings::load_config(Path::new("examples/hello")).unwrap();
        let projects = load_projects(&cfg);
        assert!(!projects.is_empty());
    }
}
