use clap::Args;
#[cfg(feature = "rustup")]
use serde::Deserialize;

use crate::external_cli::cargo::install::AutoInstall;

#[derive(Debug, Args, Clone)]
pub struct LintArgs {
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

/// Represents the contents of `rust-toolchain.toml`.
#[cfg(feature = "rustup")]
#[derive(Deserialize)]
struct RustToolchain {
    toolchain: Toolchain,
}

#[cfg(feature = "rustup")]
#[derive(Deserialize)]
struct Toolchain {
    channel: String,
}

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal. This will run [`find_bevy_lint()`] to locate `bevy_lint`.
#[cfg(feature = "rustup")]
pub fn lint(args: LintArgs) -> anyhow::Result<()> {
    use anyhow::{Context, ensure};

    use crate::external_cli::Package;

    const RUST_TOOLCHAIN: &str = include_str!("../../rust-toolchain.toml");
    const BEVY_LINT_TAG: &str = "lint-v0.4.0";
    const PACKAGE: &str = "bevy_lint";
    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    let rust_toolchain: RustToolchain =
        toml::from_str(RUST_TOOLCHAIN).context("Failed to parse `rust-toolchain.toml`.")?;

    let package = Package {
        name: PACKAGE.into(),
        required_toolchain: Some(rust_toolchain.toolchain.channel),
        git: Some(GIT_URL.to_string()),
        tag: Some(BEVY_LINT_TAG.to_string()),
        ..Default::default()
    };

    let auto_install = args.auto_install();

    let status = crate::external_cli::CommandExt::new("bevy_lint")
        .require_package(package)
        .args(args.cargo_check_args)
        .ensure_status(auto_install)?;

    ensure!(
        status.success(),
        "`bevy_lint` exited with a non-zero exit code."
    );

    Ok(())
}
