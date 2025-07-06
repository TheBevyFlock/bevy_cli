use clap::Args;

/// Arguments for creating a new Bevy project.
///
/// This subcommand allows you to generate a new Bevy project
/// using a specified template and project name.
#[derive(Args)]
pub struct NewArgs {
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

    /// The git branch to use
    #[arg(short, long, default_value = "main")]
    pub branch: String,
}
