use clap::Args;

use crate::external_cli::cargo::install::AutoInstall;

/// Arguments for creating a new Bevy project.
///
/// This subcommand allows you to generate a new Bevy project
/// using a specified template and project name.
#[derive(Args)]
pub struct NewArgs {
    /// Confirm all prompts automatically.
    #[arg(long = "yes", default_value_t = false)]
    pub confirm_prompts: bool,

    /// The desired name for the new project.
    ///
    /// This will be the name of the directory and will be used in the project's files
    pub name: String,

    /// The name of the template to use for generating the project.
    ///
    /// Templates are GitHub repositories. Any repository from the GitHub organization
    /// "TheBevyFlock" with the prefix `bevy_new_` will be usable via
    /// its shortcut form i.e. `2d` will use the template `bevy_new_2d`. Full GitHub URLs can also
    /// be passed in the template argument.
    ///
    /// Can be omitted to use a built-in template.
    #[arg(short, long, default_value = "minimal")]
    pub template: String,

    /// Branch to use when installing from git
    #[arg(short, long, conflicts_with_all = ["revision", "tag"])]
    pub branch: Option<String>,

    /// Tag to use when installing from git
    #[arg(short, long, conflicts_with_all = ["revision", "branch"])]
    pub tag: Option<String>,

    /// Git revision to use when installing from git (e.g. a commit hash)
    #[arg(short, long, conflicts_with_all = ["tag", "branch"], alias = "rev")]
    pub revision: Option<String>,

    /// Arguments to pass to `cargo-generate`
    ///
    /// Specified after `--`.
    #[clap(last = true, name = "ARGS")]
    pub forward_args: Vec<String>,
}

impl NewArgs {
    /// Whether to automatically install missing dependencies.
    pub(crate) fn auto_install(&self) -> AutoInstall {
        if self.confirm_prompts {
            AutoInstall::Always
        } else {
            AutoInstall::AskUser
        }
    }
}
