use std::path::PathBuf;

use crate::external_cli::cargo::metadata::{Metadata, Package};

#[derive(Debug, Clone)]
pub struct BinTarget {
    /// The package containing the binary.
    pub package: Package,
    /// The path to the directory in `target` which contains the binary.
    pub artifact_directory: PathBuf,
    /// The name of the binary (without any extensions).
    pub bin_name: String,
}

/// Determine which binary target should be run.
///
/// The `--package` arg narrows down the search space to the given package,
/// while the `--bin` and `--example` args determine the binary target within the selected packages.
///
/// If the search couldn't be narrowed down to a single binary,
/// the `default_run` option is taken into account.
///
/// The path to the compiled binary is determined via the compilation target and profile.
pub(crate) fn select_run_binary(
    metadata: &Metadata,
    package_name: Option<&str>,
    bin_name: Option<&str>,
    example_name: Option<&str>,
    compile_target: Option<&str>,
    compile_profile: &str,
) -> anyhow::Result<BinTarget> {
    // Determine which packages the binary could be in
    let packages = if let Some(package_name) = package_name {
        let package = metadata
            .packages
            .iter()
            .find(|package| package.name == *package_name)
            .ok_or_else(|| anyhow::anyhow!("Failed to find package {package_name}"))?;
        vec![package]
    } else {
        metadata.packages.iter().collect()
    };

    let mut is_example = false;

    let (target, package) = if let Some(bin_name) = bin_name {
        // The user specified a concrete binary
        let bins: Vec<_> = packages
            .iter()
            .flat_map(|package| {
                package.bin_targets().filter_map(move |target| {
                    if target.name == *bin_name {
                        Some((target, package))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if bins.is_empty() {
            anyhow::bail!("No binary with name {bin_name} available!");
        } else if bins.len() > 1 {
            anyhow::bail!("Multiple binaries with name {bin_name} available!");
        }

        bins[0]
    } else if let Some(example_name) = example_name {
        // The user specified a concrete example
        let examples: Vec<_> = packages
            .iter()
            .flat_map(|package| {
                package.example_targets().filter_map(move |target| {
                    if target.name == *example_name {
                        Some((target, package))
                    } else {
                        None
                    }
                })
            })
            .collect();

        if examples.is_empty() {
            anyhow::bail!("No example with name {example_name} available!");
        } else if examples.len() > 1 {
            anyhow::bail!("Multiple examples with name {example_name} available!");
        }

        is_example = true;
        examples[0]
    } else {
        // Nothing concrete specified, try to pick one automatically

        // If there is only one binary, pick that one
        let bins: Vec<_> = packages
            .iter()
            .flat_map(|package| package.bin_targets().map(move |target| (target, package)))
            .collect();

        if bins.is_empty() {
            anyhow::bail!("No binaries available!");
        } else if bins.len() == 1 {
            bins[0]
        } else {
            // Otherwise, check if there is a default run target defined
            let default_runs: Vec<_> = packages
                .iter()
                .filter_map(|package| package.default_run.as_ref())
                .collect();

            if default_runs.is_empty() {
                anyhow::bail!("There are multiple binaries available, try specifying one with --bin or define `default_run` in the Cargo.toml");
            } else if default_runs.len() > 1 {
                anyhow::bail!(
                    "Found multiple `default_run` definitions, I don't know which one to pick!"
                );
            }

            let default_run = default_runs[0];
            *bins
                .iter()
                .find(|(bin, _)| bin.name == *default_run)
                .ok_or_else(|| anyhow::anyhow!("Didn't find `default_run` binary {default_run}"))?
        }
    };

    // Assemble the path where the binary will be put
    let artifact_directory = get_artifact_directory(
        metadata.target_directory.clone(),
        compile_target,
        compile_profile,
        is_example,
    );

    Ok(BinTarget {
        package: (**package).clone(),
        bin_name: target.name.clone(),
        artifact_directory,
    })
}

/// Determine the path to the directory which contains the compilation artifacts.
fn get_artifact_directory(
    target_directory: impl Into<PathBuf>,
    target: Option<&str>,
    profile: &str,
    is_example: bool,
) -> PathBuf {
    let mut artifact_directory = target_directory.into();

    if let Some(target) = target {
        artifact_directory.push(target);
    }

    if profile == "dev" {
        // For some reason, the dev profile has a debug folder instead
        artifact_directory.push("debug");
    } else {
        artifact_directory.push(profile);
    }

    if is_example {
        artifact_directory.push("examples");
    }

    artifact_directory
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_artifact_directory_dev_native() {
        let actual = get_artifact_directory(Path::new("/target"), None, "dev", false);
        assert_eq!(actual, Path::new("/target/debug"));
    }

    #[test]
    fn test_artifact_directory_release_native() {
        let actual = get_artifact_directory(Path::new("/target"), None, "release", false);
        assert_eq!(actual, Path::new("/target/release"));
    }

    #[test]
    fn test_artifact_directory_dev_native_example() {
        let actual = get_artifact_directory(Path::new("/target"), None, "dev", true);
        assert_eq!(actual, Path::new("/target/debug/examples"));
    }

    #[test]
    fn test_artifact_directory_dev_web() {
        let actual = get_artifact_directory(
            Path::new("/target"),
            Some("wasm32-unknown-unknown"),
            "web",
            false,
        );
        assert_eq!(actual, Path::new("/target/wasm32-unknown-unknown/web"));
    }

    #[test]
    fn test_artifact_directory_release_web() {
        let actual = get_artifact_directory(
            Path::new("/target"),
            Some("wasm32-unknown-unknown"),
            "web-release",
            false,
        );
        assert_eq!(
            actual,
            Path::new("/target/wasm32-unknown-unknown/web-release")
        );
    }
}
