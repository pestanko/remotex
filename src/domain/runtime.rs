use anyhow::Ok;

use super::project::{Project, Task, TaskType};

pub async fn execute_project(proj: &Project) -> anyhow::Result<()> {
    log::info!("Executing project: {}", proj.codename);

    for task in &proj.tasks {
        execute_task(proj, task).await?;
    }

    Ok(())
}

async fn execute_task(proj: &Project, task: &Task) -> anyhow::Result<()> {
    log::info!(
        "Executing project's \"{}\" task \"{}",
        proj.codename,
        task.name
    );

    let res = match &task.value {
        TaskType::Command { name, args } => execute_command(name, args).await,
    };

    if res.is_err() {
        log::error!(
            "Task executition failed {} - {}: {:?}",
            proj.codename,
            task.name,
            res
        )
    }

    res
}

async fn execute_command(name: &str, args: &[String]) -> anyhow::Result<()> {
    log::trace!("Exec command \"{}\" with args: {:?}", name, args);

    let mut proc = tokio::process::Command::new(name).args(args).spawn()?;

    let status = proc.wait().await?;

    log::debug!("Process status: {}", status);
    log::trace!("Stdout: {:?}", proc.stdout);
    log::trace!("Stderr: {:?}", proc.stderr);

    if !status.success() {
        return Err(anyhow::anyhow!("Execution failed: {}", status));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::execute_command;

    use test_log::test;

    #[test(tokio::test)]
    async fn execute_raw_command() {
        let args = ["--version".to_string()];
        let res = execute_command("rustc", &args).await;
        assert!(res.is_ok(), "there was an error: {:?}", res);
    }
}
