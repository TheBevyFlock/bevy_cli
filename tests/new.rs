use assert_cmd::prelude::*;
use std::{path::Path, process::Command};
use tempfile::TempDir;

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

#[test]
fn should_scaffold_new_default_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "default-project";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = Command::cargo_bin("bevy")?;
    cmd.current_dir(temp_dir.path()).args(["new", project_name]);

    dbg!(cmd.output()?);

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

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

    dbg!(cmd.output()?);

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

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

    dbg!(cmd.output()?);

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}
