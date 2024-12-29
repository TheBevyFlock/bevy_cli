use std::process::{exit, Command};

use dialoguer::Confirm;

/// Check if the given program is installed on the system.
///
/// This assumes that the program offers a `--version` flag.
fn is_installed(program: &str) -> bool {
    let output = Command::new(program).arg("--version").output();

    if let Ok(output) = output {
        output.status.success()
    } else {
        false
    }
}

/// Checks if the program is installed and installs it if it isn't.
///
/// Returns `true` if the program needed to be installed.
pub(crate) fn if_needed(
    program: &str,
    package: &str,
    package_version: Option<&str>,
    ask_user: bool,
    hidden: bool,
) -> anyhow::Result<bool> {
    if is_installed(program) {
        return Ok(false);
    }

    // Abort if the user doesn't want to install it
    if ask_user
        && !Confirm::new()
            .with_prompt(format!(
                "`{program}` is missing, should I install it for you?"
            ))
            .interact()?
    {
        exit(1);
    }

    let mut cmd = Command::new(super::program());
    cmd.arg("install").arg(package);

    if let Some(version) = package_version {
        cmd.arg("--version").arg(version);
    }

    let status = if hidden {
        cmd.output()?.status
    } else {
        cmd.status()?
    };

    if status.success() {
        Ok(true)
    } else {
        Err(anyhow::anyhow!("Failed to install `{program}`."))
    }
}
