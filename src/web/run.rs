use tracing::{error, info};

use crate::{
    build::args::BuildArgs,
    external_cli::cargo,
    run::{args::RunSubcommands, RunArgs},
};

use super::{bin_target::select_run_binary, build::build_web, serve::serve};

/// Run the app in the browser.
///
/// Requires [`RunSubcommands::Web`] to be defined.
pub(crate) fn run_web(args: &RunArgs) -> anyhow::Result<()> {
    let Some(RunSubcommands::Web(web_args)) = &args.subcommand else {
        anyhow::bail!("tried to run on the web without corresponding args");
    };

    let mut build_args: BuildArgs = args.clone().into();

    // When no target is selected, search for the default-run field and append the binary name
    // as `--bin` flag to only compile the default run target
    if build_args.cargo_args.target_args.bin.is_none()
        && build_args.cargo_args.target_args.example.is_none()
    {
        let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;
        let bin_target = select_run_binary(
            &metadata,
            args.cargo_args.package_args.package.as_deref(),
            args.cargo_args.target_args.bin.as_deref(),
            args.cargo_args.target_args.example.as_deref(),
            build_args.target().as_deref(),
            build_args.profile(),
        )?;

        build_args.cargo_args.target_args.bin = Some(bin_target.bin_name);
    }

    let web_bundle = build_web(&mut build_args)?;

    let port = web_args.port;
    let url = format!("http://localhost:{port}");

    // Serving the app is blocking, so we open the page first
    if web_args.open {
        match webbrowser::open(&url) {
            Ok(()) => info!("Your app is running at <{url}>!"),
            Err(error) => {
                error!("Failed to open the browser automatically, open the app at <{url}>. (Error: {error:?}");
            }
        }
    } else {
        info!("Open your app at <{url}>!");
    }

    serve(web_bundle, port)?;

    Ok(())
}
