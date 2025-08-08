use anyhow::Context;
use reqwest::blocking::Client;
use serde::Deserialize;
use tracing::{debug, error, info};

use crate::{
    commands::lint::InstallArgs,
    external_cli::{cargo::install::AutoInstall, rustup},
};

#[derive(Deserialize, Debug)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[derive(Deserialize, Debug)]
struct Toolchain {
    channel: String,
}

// TODO: pass auto_install after: https://github.com/TheBevyFlock/bevy_cli/pull/523
pub(crate) fn install_linter(arg: &InstallArgs) -> anyhow::Result<()> {
    use std::env;

    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    // get a list of all available `bevy_lint` versions, there should always be at least one (main).
    let available_versions = list_available_releases()?;

    // A specific version was passed in the `InstallArgs`
    let (rust_toolchain, version) = if let Some(version) = &arg.version {
        // Check if the desired version exists, if not exit with an error message
        if !available_versions.contains(version) {
            error!(
                "version: {} does not exist, available versions are: {:?}",
                version, available_versions
            );
            return Ok(());
        }
        // return the required toolchain version and the name of the linter tag or `main` that
        // corresponds to the desired version.
        (lookup_toolchain_version(version)?, version.clone())
    }
    // No version was passed in the `InstallArgs` open a dialog with all available versions
    // (including the main branch) to choose from.
    else {
        let Some(selection) = dialoguer::FuzzySelect::new()
            .with_prompt("Available `bevy_lint` versions")
            .items(&available_versions)
            .interact_opt()?
        else {
            debug!("no version selected");
            return Ok(());
        };

        let version = &available_versions[selection];
        debug!("selected {}", version);
        // return the required toolchain version and the name of the linter tag or `main` that
        // corresponds to the desired version.
        (lookup_toolchain_version(version)?, version.clone())
    };

    rustup::install_toolchain_if_needed(&rust_toolchain.toolchain.channel, AutoInstall::Always)?;

    let mut cmd = crate::external_cli::CommandExt::new("rustup");

    cmd.arg("run")
        .arg(rust_toolchain.toolchain.channel)
        .arg(env::var_os("BEVY_CLI_CARGO").unwrap_or("cargo".into()))
        .arg("install")
        .arg("--git")
        .arg(GIT_URL);

    if version == "main" {
        cmd.arg("--branch").arg("main");
    } else {
        cmd.arg("--tag").arg(format!("lint-{version}"));
    }

    cmd.arg("--locked")
        .arg("bevy_lint")
        .ensure_status(AutoInstall::Never)
        .context(format!("failed to install `bevy_lint-{version}`"))?;

    Ok(())
}

pub(crate) fn list() -> anyhow::Result<()> {
    use std::fmt::Write;
    let releases = list_available_releases()?;

    // TODO: make this pretty
    let mut output = String::new();

    writeln!(output, "╔═════╤══════════════╗")?;
    writeln!(output, "║  #  │ Version      ║")?;
    writeln!(output, "╟─────┼──────────────╢")?;

    for (i, version) in releases.iter().enumerate() {
        writeln!(output, "║ {:>2}  │ {:<12} ║", i + 1, version)?;
    }

    writeln!(output, "╚═════╧══════════════╝")?;

    info!("Available `bevy_lint` versions:\n{output}");

    Ok(())
}

// Lists the available `bevy_lint` releases from the `GitHub` release page (including main).
fn list_available_releases() -> anyhow::Result<Vec<String>> {
    #[derive(Deserialize, Debug)]
    struct Release {
        name: String,
    }

    const URL: &str = "https://api.github.com/repos/TheBevyFlock/bevy_cli/releases";

    let releases = Client::new()
        .get(URL)
        .header("User-Agent", "bevy_cli")
        .send()
        .context("failed to query available GitHub releases")?
        .json::<Vec<Release>>()?;

    let mut releases = releases
        .iter()
        .filter_map(|r| {
            r.name
                .strip_prefix("`bevy_lint` - ")
                .map(ToString::to_string)
        })
        .collect::<Vec<_>>();

    releases.push("main".to_string());

    Ok(releases)
}

/// Looks up the `rust-toolchain.toml` file for the given version from GitHub and tries to parse it
/// into [`RustToolchain`].
fn lookup_toolchain_version(linter_version: &str) -> anyhow::Result<RustToolchain> {
    let url = if linter_version == "main" {
        "https://raw.githubusercontent.com/TheBevyFlock/bevy_cli/main/rust-toolchain.toml"
            .to_string()
    } else {
        // the releases are named <`bevy_lint`-v0.3.0> but tags are only named <lint-v0.3.0>, so
        // append `lint-`
        format!(
            "https://raw.githubusercontent.com/TheBevyFlock/bevy_cli/lint-{linter_version}/rust-toolchain.toml"
        )
    };

    let response = Client::new()
        .get(url)
        .header("User-Agent", "bevy_cli")
        .send()
        .context(
            "failed to query `rust-toolchain.toml` from GitHub for the given `bevy_lint` version",
        )?
        .text()?;

    let rust_toolchain: RustToolchain =
        toml::from_str(&response).context("Failed to parse `rust-toolchain.toml`.")?;

    Ok(rust_toolchain)
}
