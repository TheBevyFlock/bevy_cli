//! All available commands for the Bevy CLI.

use cargo_metadata::{Metadata, Package, TargetKind};

pub mod build;
pub mod completions;
pub mod lint;
pub mod new;
pub mod run;

fn get_default_package<'m>(
    metadata: &'m Metadata,
    package_arg: Option<&String>,
    run_command: bool,
) -> anyhow::Result<Option<&'m Package>> {
    let workspace_packages = metadata.workspace_packages();
    // If the `--package` arg was passed, search for the given package, otherwise
    // check if the current directory contains a package.
    let package = if let Some(package_name) = package_arg {
        metadata
            .packages
            .iter()
            .find(|package| package.name.as_str() == package_name)
    } else if run_command {
        // If there is only one binary, pick that one
        let bins: Vec<_> = workspace_packages
            .iter()
            .flat_map(|package| {
                package
                    .targets
                    .iter()
                    .filter(|target| target.kind.contains(&TargetKind::Bin))
                    .map(move |_| package)
            })
            .collect();

        let bin_package = if bins.is_empty() {
            anyhow::bail!("No binaries available!");
        } else if bins.len() == 1 {
            bins[0]
        } else {
            // Otherwise, check if there is a default run target defined
            let default_runs: Vec<_> = workspace_packages
                .iter()
                .filter_map(|package| package.default_run.as_ref())
                .collect();

            if default_runs.is_empty() {
                anyhow::bail!(
                    "There are multiple binaries available, try one of the following:
- add `--bin` or `--package` after `bevy run` to specify which binary or package to run,
- define `default-run` in the Cargo.toml to define the default binary that should be executed in a package,
- define `default-members` in the Cargo.toml of your workspace to define the default package to pick the binary from."
                );
            } else if default_runs.len() > 1 {
                anyhow::bail!(
                    "Found multiple `default-run` definitions, I don't know which one to pick!"
                );
            }

            let default_run = default_runs[0];
            **bins
                .iter()
                .find(|bin| bin.name == *default_run)
                .ok_or_else(|| anyhow::anyhow!("Didn't find `default-run` binary {default_run}"))?
        };
        Some(bin_package)
    } else {
        // Get the current directory
        let current_dir = std::env::current_dir()?;

        // Find the package whose manifest_path matches the current directory
        metadata.packages.iter().find(|pkg| {
            pkg.manifest_path
                .parent()
                .map(cargo_metadata::camino::Utf8Path::as_std_path)
                == Some(&current_dir)
        })
    };

    Ok(package)
}
