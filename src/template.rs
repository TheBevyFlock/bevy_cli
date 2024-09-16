use cargo_generate::{GenerateArgs, TemplatePath};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Repository {
    html_url: String,
    name: String,
}

/// Generates a new template to the returned [`PathBuf`] using the given name and Git repository.
///
/// If `git` is [`None`], it will default to [TheBevyFlock/bevy_quickstart].
///
/// [TheBevyFlock/bevy_quickstart]: https://github.com/TheBevyFlock/bevy_quickstart
pub fn generate_template(name: &str, template: &str, branch: &str) -> anyhow::Result<PathBuf> {
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
/// If `template` is [`None`], it will default to `bevy_new_default`.
fn template_path(template: &str, branch: &str) -> anyhow::Result<TemplatePath> {
    const TEMPLATE_ORG: &str = "TheBevyFlock";
    const TEMPLATE_PREFIX: &str = "bevy_new_";

    let templates = fetch_template_repositories(TEMPLATE_ORG, TEMPLATE_PREFIX)?;
    let maybe_url = templates.iter().find_map(|r| {
        // Does the provided argument match any of our existing templates?
        let suffix = &r.name[TEMPLATE_PREFIX.len()..];
        if suffix == template {
            return Some(r.html_url.clone());
        }
        None
    });

    let Some(url) = maybe_url else {
        anyhow::bail!(
            "Could not retrieve the requested template: {}. Check the template name.",
            template
        );
    };

    Ok(TemplatePath {
        git: Some(url),
        branch: Some(branch.into()),
        ..Default::default()
    })
}

/// Returns a list of GitHub repositories with the prefix `bevy_new_` in the given GitHub org.
fn fetch_template_repositories(org: &str, prefix: &str) -> anyhow::Result<Vec<Repository>> {
    let url = format!("https://api.github.com/orgs/{}/repos", org);

    let client = Client::new();
    let repos: Vec<Repository> = client
        .get(&url)
        .header("User-Agent", "bevy_cli")
        .send()?
        .json()?;

    let templates: Vec<Repository> = repos
        .into_iter()
        .filter(|repo| repo.name.starts_with(prefix))
        .collect();

    Ok(templates)
}
