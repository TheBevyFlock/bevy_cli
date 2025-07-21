use anyhow::{Context, ensure};
use clap::{Args, Subcommand};
use reqwest::blocking::Client;
#[cfg(feature = "rustup")]
use serde::Deserialize;
use tracing::{debug, error, info};

use crate::external_cli::{cargo::install::AutoInstall, rustup};

#[derive(Debug, Args)]
pub struct LintArgs {
    #[command(subcommand)]
    pub subcommand: Option<LintSubcommands>,
    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub confirm_prompts: bool,

    /// Arguments to forward to `cargo check`.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub cargo_check_args: Vec<String>,
}

impl LintArgs {
    /// Whether to automatically install missing dependencies.
    // Only needed with the `rustup` feature
    #[allow(dead_code)]
    pub(crate) fn auto_install(&self) -> AutoInstall {
        if self.confirm_prompts {
            AutoInstall::Always
        } else {
            AutoInstall::AskUser
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum LintSubcommands {
    List,
    Install(InstallArgs),
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    version: Option<String>,
}

/// Represents the contents of `rust-toolchain.toml`.
#[cfg(feature = "rustup")]
#[derive(Deserialize, Debug)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[cfg(feature = "rustup")]
#[derive(Deserialize, Debug)]
struct Toolchain {
    channel: String,
}

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal. This will run [`find_bevy_lint()`] to locate `bevy_lint`.
#[cfg(feature = "rustup")]
pub fn lint(args: LintArgs) -> anyhow::Result<()> {
    if let Some(subcommand) = &args.subcommand {
        return match subcommand {
            LintSubcommands::List => list(),
            LintSubcommands::Install(arg) => install_linter(arg),
        };
    }

    let auto_install = args.auto_install();

    // TODO: What do we want to autoinstall if `bevy_lint` is not present?
    let status = crate::external_cli::CommandExt::new("bevy_lint")
        .args(args.cargo_check_args)
        .ensure_status(auto_install)?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

fn install_linter(arg: &InstallArgs) -> anyhow::Result<()> {
    use std::env;

    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    // get a list of all available `bevy_lint` versions, there should always be at least one.
    let available_versions = list_available_releases()?;

    let (rust_toolchain, version) = if let Some(version) = &arg.version {
        // Check if the desired version exists, if not exit with an error message
        if !available_versions.contains(version) {
            error!(
                "version: {} does not exist, available versions are: {:?}",
                version, available_versions
            );
            return Ok(());
        }

        (lookup_toolchain_version(version)?, version.clone())
    } else {
        // Create a `FuzzySelect` select dialog with all the available `bevy_lint` versions
        // (including main).
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

    let status = cmd
        .arg("--locked")
        .arg("bevy_lint")
        .ensure_status(AutoInstall::Always)?;

    ensure!(
        status.success(),
        "installing `bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}

fn list() -> anyhow::Result<()> {
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
    let url = "https://api.github.com/repos/TheBevyFlock/bevy_cli/releases";
    let client = Client::new();
    let releases = client
        .get(url)
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

    let client = Client::new();

    let response = &client
        .get(url)
        .header("User-Agent", "bevy_cli")
        .send()
        .context(
            "failed to query `rust-toolchain.toml` from GitHub for the given `bevy_lint` version",
        )?
        .text()?;

    let rust_toolchain: RustToolchain =
        toml::from_str(response).context("Failed to parse `rust-toolchain.toml`.")?;

    Ok(rust_toolchain)
}
