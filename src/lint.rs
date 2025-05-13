use clap::Args;

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

/// Runs `bevy_lint`, if it is installed, with the given arguments.
///
/// Calling `lint(vec!["--workspace"])` is equivalent to calling `bevy_lint --workspace` in the
/// terminal. This will run [`find_bevy_lint()`] to locate `bevy_lint`.
#[cfg(feature = "rustup")]
pub fn lint(args: LintArgs) -> anyhow::Result<()> {
    use crate::external_cli::Package;
    use anyhow::{Context, ensure};
    use toml_edit::DocumentMut;

    const RUST_TOOLCHAIN: &str = include_str!("../rust-toolchain.toml");
    const BEVY_LINT_TAG: &str = "lint-v0.3.0";
    const PACKAGE: &str = "bevy_lint";
    const GIT_URL: &str = "https://github.com/TheBevyFlock/bevy_cli.git";

    let rust_toolchain = RUST_TOOLCHAIN
        .parse::<DocumentMut>()
        .context("Failed to parse `rust-toolchain.toml`.")?;

    let channel = rust_toolchain["toolchain"]["channel"]
        .as_str()
        .context("Could not find `toolchain.channel` key in `rust-toolchain.toml`.")?;

    let package = Package {
        name: PACKAGE.into(),
        required_toolchain: Some(channel.to_string()),
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
