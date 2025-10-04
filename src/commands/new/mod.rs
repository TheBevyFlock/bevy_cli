//! Utilities to create a new Bevy project with `cargo-generate`

pub use args::*;
use serde::Deserialize;

use crate::external_cli::{CommandExt, Package};

mod args;

/// An abbreviated version of the full [GitHub API response](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-organization-repositories).
///
/// Note that `html_url` is the correct value to use for cloning repositories. By contrast, `url`
/// is an api.github.com URL that will not work for cloning.
#[derive(Debug, Deserialize)]
struct Repository {
    html_url: String,
    name: String,
}

/// Generates a new template using the given name and Git repository.
///
/// This will default to [`TheBevyFlock/bevy_new_minimal`] when no template is specified in the CLI.
///
/// [`TheBevyFlock/bevy_new_minimal`]: https://github.com/TheBevyFlock/bevy_new_miminal
pub fn new(args: &NewArgs) -> anyhow::Result<()> {
    const PROGRAM: &str = "cargo-generate";
    // Validate that the package name starts with an alphabetic character
    if let Some(first_char) = args.name.chars().next() {
        anyhow::ensure!(
            first_char.is_alphabetic(),
            "invalid character `{first_char}` in package name: {}",
            args.name
        );
    }

    let Some(git) = expand_builtin(args.template.as_str())?
        .or(expand_github_shortform(args.template.as_str()))
        .or(Some(args.template.clone()))
    else {
        return Ok(());
    };

    let mut cmd = CommandExt::new(PROGRAM);

    cmd.arg("generate");

    cmd.args(["--git", git.as_str()]);

    match (&args.branch, &args.tag, &args.revision) {
        (Some(branch), None, None) => {
            cmd.args(["--branch", branch]);
        }
        (None, Some(tag), None) => {
            cmd.args(["--tag", tag]);
        }
        (None, None, Some(rev)) => {
            cmd.args(["--rev", rev]);
        }
        // Use `cargo-generate`'s default behavior, which usually means installing the default
        // branch.
        (None, None, None) => {}
        _ => unreachable!("clap enforces, that only one of the options can be set"),
    }

    cmd.args(args.forward_args.iter());

    cmd.args(["--name", args.name.as_str()])
        .require_package(Package {
            name: PROGRAM.into(),
            ..Default::default()
        })
        .ensure_status(args.auto_install())?;

    Ok(())
}

/// Attempts to match one of our builtin templates by retrieving all repos from TheBevyFlock
/// prefixed with `bevy_new_`.
fn expand_builtin(template: &str) -> anyhow::Result<Option<String>> {
    const TEMPLATE_ORG: &str = "TheBevyFlock";
    const TEMPLATE_PREFIX: &str = "bevy_new_";

    let templates = fetch_template_repositories(TEMPLATE_ORG, TEMPLATE_PREFIX)?;
    let maybe_builtin = templates.iter().find_map(|r| {
        // Does the provided argument match any of our existing templates?
        let suffix = &r.name[TEMPLATE_PREFIX.len()..];
        (suffix == template).then(|| r.html_url.clone())
    });

    Ok(maybe_builtin)
}

/// If the template argument has org/repo format using GitHub's allowed characters for both,
/// attempt to expand it into a GitHub URL.
fn expand_github_shortform(template: &str) -> Option<String> {
    is_repo_shortform(template).then(|| format!("https://github.com/{template}.git"))
}

/// Determine if the argument is a shorthand for a repository.
/// The shorthand has the form `org/repo`.
fn is_repo_shortform(template: &str) -> bool {
    fn is_valid_char(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.'
    }

    let Some((org, repo)) = template.split_once('/') else {
        return false;
    };

    !org.is_empty()
        && org.chars().all(is_valid_char)
        && !repo.is_empty()
        && repo.chars().all(is_valid_char)
}

/// Returns a list of GitHub repositories with the prefix `bevy_new_` in the given GitHub org.
fn fetch_template_repositories(org: &str, prefix: &str) -> anyhow::Result<Vec<Repository>> {
    let url = format!("https://api.github.com/orgs/{org}/repos");

    let repos: Vec<Repository> = ureq::get(&url)
        .header(
            "User-Agent",
            format!(
                "bevy_cli/{} (https://thebevyflock.github.io/bevy_cli)",
                env!("CARGO_PKG_VERSION")
            ),
        )
        .header("Accept", "application/json")
        .call()?
        .body_mut()
        .read_json()?;

    let templates: Vec<Repository> = repos
        .into_iter()
        .filter(|repo| repo.name.starts_with(prefix))
        .collect();

    Ok(templates)
}

#[cfg(test)]
mod tests {
    mod is_github_template {
        use super::super::is_repo_shortform;

        #[test]
        fn true_for_github_template() {
            assert!(is_repo_shortform("TheBevyFlock/bevy_new_minimal"));
        }

        #[test]
        fn false_for_url() {
            assert!(!is_repo_shortform(
                "https://github.com/TheBevyFlock/bevy_new_2d"
            ));
        }

        #[test]
        fn false_for_name() {
            assert!(!is_repo_shortform("2d"));
        }
    }
}
