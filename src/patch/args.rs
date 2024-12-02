use clap::Args;

#[derive(Args, Debug)]
pub struct PatchArgs {
    /// The URL to the Git repository.
    #[arg(long)]
    pub git: String,
    
    #[clap(flatten)]
    pub git_revision_args: GitRevisionArgs,
}

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct GitRevisionArgs {
    /// The branch of the Git repository to use.
    #[clap(long)]
    pub branch: Option<String>,

    /// The tag of the Git repository to use.
    #[clap(long)]
    pub tag: Option<String>,

    /// The revision (commit or reference) of the Git repository to use.
    #[clap(long)]
    pub rev: Option<String>,
}
