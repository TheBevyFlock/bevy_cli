use cargo_generate::{GenerateArgs, TemplatePath};
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::PathBuf;
use tracing::debug;

/// An abbreviated version of the full [GitHub API response](https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-organization-repositories).
///
/// Note that `html_url` is the correct value to use for cloning repositories. By contrast, `url`
/// is an api.github.com URL that will not work for cloning.
#[derive(Debug, Deserialize)]
struct Repository {
    html_url: String,
    name: String,
}

/// Generates a new template to the returned [`PathBuf`] using the given name and Git repository.
///
/// If `git` is [`None`], it will default to [TheBevyFlock/bevy_new_minimal].
///
/// [TheBevyFlock/bevy_new_minimal]: https://github.com/TheBevyFlock/bevy_new_miminal
pub fn generate_template(name: &str, template: &str, branch: &str) -> anyhow::Result<PathBuf> {
    debug!("generating template project called {name} with template: {template}:{branch}");
    cargo_generate::generate(GenerateArgs {
        template_path: template_path(template, branch)?,
        name: Some(name.to_string()),
        // prevent conversion to kebab-case
        force: true,
        ..Default::default()
    })
}

/// Returns the [`TemplatePath`] for a given Git repository.
///
/// If a shortcut is provided, e.g. `2d`, we will attempt to expand it to `bevy_new_2d`. (This value
/// defaults to `minimal`.)
/// If an org/repo shortform is provided, we will attempt to expand it to a URL.
/// Otherwise, we pass the value directly to `cargo-generate`, presuming it to be a URL.
fn template_path(template: &str, branch: &str) -> anyhow::Result<TemplatePath> {
    let git = expand_builtin(template)?
        .or(expand_github_shortform(template))
        .or(Some(template.into()));

    debug!("{git:?}");

    Ok(TemplatePath {
        git,
        branch: Some(branch.into()),
        ..Default::default()
    })
}

/// Attempts to match one of our builtin templates by retrieving all repos from TheBevyFlock
/// prefixed with `bevy_new_`.
fn expand_builtin(template: &str) -> anyhow::Result<Option<String>> {
    const TEMPLATE_ORG: &str = "TheBevyFlock";
    const TEMPLATE_PREFIX: &str = "bevy_new_";

    let templates = fetch_template_repositories(TEMPLATE_ORG, TEMPLATE_PREFIX)?;
    debug!("all builtin templates: {templates:?}");
    let maybe_builtin = templates.iter().find_map(|r| {
        // Does the provided argument match any of our existing templates?
        let suffix = &r.name[TEMPLATE_PREFIX.len()..];
        (suffix == template).then(|| r.html_url.clone())
    });
    debug!("matches buildint template: {maybe_builtin:?}");

    Ok(maybe_builtin)
}

/// If the template argument has org/repo format using GitHub's allowed characters for both,
/// attempt to expand it into a GitHub URL.
fn expand_github_shortform(template: &str) -> Option<String> {
    let re = Regex::new(r"^[a-zA-Z0-9_\.\-]+/[a-zA-Z0-9_\-\.]+$").unwrap();
    re.is_match(template)
        .then(|| format!("https://github.com/{template}.git"))
}

/// Returns a list of GitHub repositories with the prefix `bevy_new_` in the given GitHub org.
fn fetch_template_repositories(org: &str, prefix: &str) -> anyhow::Result<Vec<Repository>> {
    debug!("fetching template repositories");
    let url = format!("https://api.github.com/orgs/{org}/repos");

    let client = Client::new();
    let repos: Vec<Repository> = client
        .get(&url)
        .header("User-Agent", "bevy_cli/1.0.0")
        .send()?
        .json()?;

    debug!("fetched repos: {repos:?}");

    let templates: Vec<Repository> = repos
        .into_iter()
        .filter(|repo| repo.name.starts_with(prefix))
        .collect();

    debug!("fetched templates: {templates:?}");

    Ok(templates)
}
