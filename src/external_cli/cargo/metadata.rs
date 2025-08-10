use std::{ffi::OsStr, str::from_utf8};

use anyhow::Context;
use cargo_metadata::{Metadata, MetadataCommand};
use tracing::Level;

use super::install::AutoInstall;
use crate::external_cli::CommandExt;

/// Create a command to run `cargo metadata`.
pub(crate) fn command() -> CommandExt {
    let mut command = CommandExt::from_command(MetadataCommand::new().cargo_command());
    command.log_level(Level::DEBUG);
    command
}

/// Try to obtain the Cargo metadata of this package.
pub(crate) fn metadata() -> anyhow::Result<Metadata> {
    metadata_with_args::<[&str; 0], &str>([])
}

/// Try to obtain the Cargo metadata of this package.
///
/// To see which additional args are available, [consult the `cargo metadata` documentation](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html)
/// or use `cargo metadata --help`.
pub(crate) fn metadata_with_args<I, S>(additional_args: I) -> anyhow::Result<Metadata>
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let output = command()
        .args(additional_args)
        .output(AutoInstall::Never)
        .context("`bevy_cli` failed to obtain package metadata")?;

    let stdout = from_utf8(&output.stdout)?
        .lines()
        .find(|line| line.starts_with('{'))
        .ok_or(anyhow::anyhow!("stdout is not valid json"))?;

    Ok(cargo_metadata::MetadataCommand::parse(stdout)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_determine_metadata_of_this_package() {
        let metadata = metadata();
        assert!(metadata.is_ok());
        let metadata = metadata.unwrap();

        assert!(
            metadata
                .packages
                .iter()
                .any(|package| package.name.as_str() == "bevy_cli")
        );
    }
}
