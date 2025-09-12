use cargo_test_support::{cargo_test, file, project, str};

use crate::prelude::bevy_exe;

#[cargo_test]
fn foo() {
    let project = project().build();

    snapbox::cmd::Command::new(bevy_exe())
        .arg("lint")
        // Remove `PATH` environmental variable so that the CLI cannot find the linter.
        .env_remove("PATH")
        .current_dir(project.root())
        .assert()
        .failure()
        .stdout_eq(file!["stdout.term.svg"])
        .stderr_eq(str![]);
}
