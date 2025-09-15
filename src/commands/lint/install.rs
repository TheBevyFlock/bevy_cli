use anyhow::Context;
use reqwest::blocking::Client;
use serde::Deserialize;
#[cfg(feature = "rustup")]
use tracing::debug;

#[cfg(feature = "rustup")]
use crate::{
    commands::lint::InstallArgs,
    external_cli::{CommandExt, cargo::install::AutoInstall},
};
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Toolchain {
    channel: String,
    components: Vec<String>,
}

#[cfg(feature = "rustup")]
pub(crate) fn install_linter(args: &InstallArgs) -> anyhow::Result<()> {
    use std::env;

    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    // Get a list of all available `bevy_lint` versions, there should always be at least one (main).
    let available_versions = list_available_releases()?;

    // A specific version was passed in the `InstallArgs`
    let (rust_toolchain, version) = if let Some(version) = &args.version {
        // Check if the desired version exists, if not return with an error message
        if !available_versions.contains(version) {
            anyhow::bail!(
                "version `{}` does not exist. Available versions: {:?}",
                version,
                available_versions
            );
        }
        // Return the required toolchain version and the name of the linter tag or `main` that
        // corresponds to the desired version.
        (lookup_toolchain_version(version)?, version)
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

        let required_toolchain = lookup_toolchain_version(version)?;

        // Return the required toolchain version and the name of the linter tag or `main` that
        // corresponds to the desired version.
        (required_toolchain, version)
    };

    if !args.auto_install().confirm(format!(
        "Do you want to install `bevy_lint-{version}` and the required toolchain: `{}` ?",
        rust_toolchain.toolchain.channel
    ))? {
        anyhow::bail!(
            "User does not want to install `bevy_lint-{version}` and the required toolchain: `{}`",
            rust_toolchain.toolchain.channel
        );
    }

    install_toolchain(&rust_toolchain)?;

    // Prepare command to install `bevy_lint`
    let mut cmd = CommandExt::new("rustup");

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

/// Print the available `bevy_lint` versions, including main, in a table to stdout.
pub(crate) fn list() -> anyhow::Result<()> {
    let releases = list_available_releases()?;

    let mut table = comfy_table::Table::new();

    table
        .load_preset(comfy_table::presets::UTF8_FULL)
        .apply_modifier(comfy_table::modifiers::UTF8_ROUND_CORNERS)
        .set_header(["Bevy Lint Version"]);

    for release in releases {
        table.add_row([release]);
    }

    println!("{table}");

    Ok(())
}

/// Lists the available `bevy_lint` releases from the GitHub release page (including main).
///
/// The main branch is always the first item in the Vector.
fn list_available_releases() -> anyhow::Result<Vec<String>> {
    #[derive(Deserialize, Debug)]
    struct Release {
        name: String,
    }

    const URL: &str = "https://api.github.com/repos/TheBevyFlock/bevy_cli/releases";

    let releases = Client::new()
        .get(URL)
        .header(
            "User-Agent",
            format!(
                "bevy_cli/{} (https://thebevyflock.github.io/bevy_cli)",
                env!("CARGO_PKG_VERSION")
            ),
        )
        .send()
        .context("failed to query available GitHub releases")?
        .json::<Vec<Release>>()?;

    Ok(std::iter::once("main".to_owned())
        .chain(
            releases
                .iter()
                .filter_map(|r| r.name.strip_prefix("`bevy_lint` - ").map(str::to_owned)),
        )
        .collect())
}

/// Looks up the `rust-toolchain.toml` file for the given version from GitHub and tries to parse it
/// into [`RustToolchain`].
#[cfg(feature = "rustup")]
fn lookup_toolchain_version(linter_version: &str) -> anyhow::Result<RustToolchain> {
    let url = if linter_version == "main" {
        "https://raw.githubusercontent.com/TheBevyFlock/bevy_cli/main/rust-toolchain.toml"
            .to_owned()
    } else {
        // the releases are named <`bevy_lint`-v0.3.0> but tags are only named <lint-v0.3.0>, so
        // append `lint-`
        format!(
            "https://raw.githubusercontent.com/TheBevyFlock/bevy_cli/lint-{linter_version}/rust-toolchain.toml"
        )
    };

    let response = Client::new()
        .get(url)
        .header(
            "User-Agent",
            format!(
                "bevy_cli/{} (https://thebevyflock.github.io/bevy_cli)",
                env!("CARGO_PKG_VERSION")
            ),
        )
        .send()
        .context(
            "failed to query `rust-toolchain.toml` from GitHub for the given `bevy_lint` version",
        )?
        .text()?;

    let rust_toolchain: RustToolchain =
        toml::from_str(&response).context("Failed to parse `rust-toolchain.toml`.")?;

    Ok(rust_toolchain)
}

/// Install the [`RustToolchain`] with its components.
#[cfg(feature = "rustup")]
fn install_toolchain(rust_toolchain: &RustToolchain) -> anyhow::Result<()> {
    let mut cmd = CommandExt::new("rustup");

    cmd.arg("toolchain")
        .arg("install")
        .arg(&rust_toolchain.toolchain.channel);

    for component in &rust_toolchain.toolchain.components {
        cmd.args(["--component", component]);
    }

    cmd.ensure_status(AutoInstall::Always).context(format!(
        "failed to install toolchain `{}`",
        rust_toolchain.toolchain.channel
    ))?;

    Ok(())
}
