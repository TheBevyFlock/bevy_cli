use cargo_generate::{GenerateArgs, TemplatePath};
use std::path::PathBuf;

/// Generates a new template to the returned [`PathBuf`] using the given name and Git repository.
///
/// If `git` is [`None`], it will default to [TheBevyFlock/bevy_quickstart].
///
/// [TheBevyFlock/bevy_quickstart]: https://github.com/TheBevyFlock/bevy_quickstart
pub fn generate_template(name: &str, git: Option<&str>) -> anyhow::Result<PathBuf> {
    cargo_generate::generate(GenerateArgs {
        template_path: template_path(git),
        name: Some(name.to_string()),
        // prevent conversion to kebab-case
        force: true,
        ..Default::default()
    })
}

/// Returns the [`TemplatePath`] for a given Git repository.
///
/// If `git` is [`None`], it will default to `bevy_quickstart`.
fn template_path(git: Option<&str>) -> TemplatePath {
    const DEFAULT_REPOSITORY: &str = "https://github.com/TheBevyFlock/bevy_quickstart.git";
    const DEFAULT_BRANCH: &str = "cargo-generate";

    if let Some(template) = git {
        TemplatePath {
            git: Some(template.to_string()),
            ..Default::default()
        }
    } else {
        TemplatePath {
            git: Some(DEFAULT_REPOSITORY.to_string()),
            branch: Some(DEFAULT_BRANCH.to_string()),
            ..Default::default()
        }
    }
}
