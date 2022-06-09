use std::path::Path;

use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
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
}
