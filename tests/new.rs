use anyhow::anyhow;
use assert_cmd::prelude::*;
use std::{path::Path, process::Command, time::Duration};
use tempfile::TempDir;
use wait_timeout::ChildExt;

fn temp_test_dir() -> anyhow::Result<TempDir> {
    Ok(tempfile::tempdir()?)
}

fn ensure_path_exists<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    anyhow::ensure!(
        path.as_ref().exists(),
        "Expected path {} does not exist",
        path.as_ref().display()
    );
    Ok(())
}

/// Spawns a child process for the given [`Command`]. If the child process does not exit
/// before the specified `timeout` duration, the child process is aborted, and a new one is spawned
/// for a maximum of `retry_count` retries.
fn run_cmd_with_timeout_and_retry(
    mut cmd: Command,
    timeout: Duration,
    retry_count: usize,
) -> anyhow::Result<()> {
    for i in 0..retry_count {
        let mut child = cmd.spawn()?;
        if let Some(exit_code) = child.wait_timeout(timeout).unwrap() {
            if exit_code.success() {
                println!("exited after {i} tries");
                return Ok(());
            }
            println!("didnt successfully exit, got exit code: {exit_code:?}");
        }
        //process didn't exit in time, stop it
        child.kill()?;
        println!("failed to exit proces in the duration: {timeout:?}");
    }
    Err(anyhow!(
        "failed to execute command: {cmd:?}, retried {retry_count} times"
    ))
}

#[test]
fn should_scaffold_new_default_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "default-project";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("bevy")?;
    cmd.current_dir(temp_dir.path()).args(["new", project_name]);

    run_cmd_with_timeout_and_retry(cmd, Duration::from_secs(5), 10)?;

    //ensure_path_exists(&project_path)?;

    //ensure_path_exists(project_path.join("Cargo.toml"))?;

    //ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}

#[test]
fn should_scaffold_new_with_minimal_template_shortcut_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "minimal-project-shortcut";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("bevy")?;
    cmd.current_dir(temp_dir.path())
        .args(["new", project_name, "-t", "minimal"]);

    run_cmd_with_timeout_and_retry(cmd, Duration::from_secs(5), 10)?;

    //ensure_path_exists(&project_path)?;

    //ensure_path_exists(project_path.join("Cargo.toml"))?;

    //ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}

#[test]
fn should_scaffold_new_with_minimal_template_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "minimal-project";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("bevy")?;
    cmd.current_dir(temp_dir.path()).args([
        "new",
        project_name,
        "-t",
        "https://github.com/TheBevyFlock/bevy_new_minimal",
    ]);

    run_cmd_with_timeout_and_retry(cmd, Duration::from_secs(5), 10)?;

    //ensure_path_exists(&project_path)?;

    //ensure_path_exists(project_path.join("Cargo.toml"))?;

    //ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}
