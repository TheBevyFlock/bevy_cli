use std::path::Path;

use assert_cmd::cargo::cargo_bin_cmd;
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
#[ignore = "Test spuriously fails in CI, please see #309."]
fn should_scaffold_new_default_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "default-project";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(temp_dir.path()).args(["new", project_name]);

    cmd.output()?;

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}

#[test]
#[ignore = "Test spuriously fails in CI, please see #309."]
fn should_scaffold_new_with_minimal_template_shortcut_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "minimal-project-shortcut";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(temp_dir.path())
        .args(["new", project_name, "-t", "minimal"]);

    cmd.output()?;

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}

#[test]
#[ignore = "Test spuriously fails in CI, please see #309."]
fn should_scaffold_new_with_minimal_template_project() -> anyhow::Result<()> {
    let temp_dir = temp_test_dir()?;
    let project_name = "minimal-project";
    let project_path = temp_dir.path().join(project_name);

    let mut cmd = cargo_bin_cmd!("bevy");
    cmd.current_dir(temp_dir.path()).args([
        "new",
        project_name,
        "-t",
        "https://github.com/TheBevyFlock/bevy_new_minimal",
    ]);

    cmd.output()?;

    ensure_path_exists(&project_path)?;

    ensure_path_exists(project_path.join("Cargo.toml"))?;

    ensure_path_exists(project_path.join("src").join("main.rs"))?;

    Ok(())
}
