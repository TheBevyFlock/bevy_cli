use args::RunSubcommands;

use crate::{
    build::ensure_web_setup,
    external_cli::{
        cargo::{
            self,
            metadata::{Metadata, Package},
            run::CargoRunArgs,
        },
        wasm_bindgen, CommandHelpers,
    },
};

pub use self::args::RunArgs;

mod args;
mod serve;

pub fn run(args: &RunArgs) -> anyhow::Result<()> {
    let cargo_args = args.cargo_args_builder();

    if let Some(RunSubcommands::Web(web_args)) = &args.subcommand {
        ensure_web_setup()?;

        let metadata = cargo::metadata::metadata_with_args(["--no-deps"])?;

        // If targeting the web, run a web server with the WASM build
        println!("Building for WASM...");
        cargo::build::command().args(cargo_args).ensure_status()?;

        println!("Bundling for the web...");
        let package_name = select_run_package(&metadata, &args.cargo_args)?
            .name
            .clone();
        wasm_bindgen::bundle(&package_name, args.profile())?;

        let port = web_args.port;
        let url = format!("http://localhost:{port}");

        // Serving the app is blocking, so we open the page first
        if web_args.open {
            match webbrowser::open(&url) {
                Ok(()) => println!("Your app is running at <{url}>!"),
                Err(error) => {
                    println!("Failed to open the browser automatically, open the app at <{url}>. (Error: {error:?}")
                }
            }
        } else {
            println!("Open your app at <{url}>!");
        }

        serve::serve(port, args.profile())?;
    } else {
        // For native builds, wrap `cargo run`
        cargo::run::command().args(cargo_args).ensure_status()?;
    }

    Ok(())
}

/// Determine which package should be run.
fn select_run_package<'a>(
    metadata: &'a Metadata,
    args: &CargoRunArgs,
) -> anyhow::Result<&'a Package> {
    let package_name = if let Some(bin) = &args.target_args.bin {
        bin.clone()
    } else if let Some(package) = &args.package_args.package {
        package.clone()
    } else {
        // Try to determine the run package automatically
        let default_runs: Vec<_> = metadata
            .packages
            .iter()
            .filter_map(|package| package.default_run.clone())
            .collect();
        anyhow::ensure!(default_runs.len() <= 1, "More than one default run target");

        if let Some(default_run) = default_runs.into_iter().next() {
            default_run
        } else {
            // If there is only one package with binary target, use that
            let bin_packages: Vec<_> = metadata
                .packages
                .iter()
                .filter(|package| package.has_bin())
                .collect();
            anyhow::ensure!(
                bin_packages.len() <= 1,
                "Multiple binary targets found: {}\nPlease select one with the `--bin` argument",
                bin_packages
                    .iter()
                    .map(|package| package.name.clone())
                    .collect::<Vec<String>>()
                    .join(", ")
            );

            anyhow::ensure!(bin_packages.len() == 1, "No binary target found");
            bin_packages[0].name.clone()
        }
    };

    match metadata
        .packages
        .iter()
        .find(|package| package.name == package_name)
    {
        Some(package) => Ok(package),
        None => Err(anyhow::anyhow!("Didn't find package {package_name}")),
    }
}
