mod args;

use crate::external_cli::cargo;

pub use self::args::PatchArgs;

pub fn patch(args: &PatchArgs) -> anyhow::Result<()> {
    let metadata = cargo::metadata::metadata()?;

    todo!();

    Ok(())
}
