use args::RunSubcommands;

use crate::{
    build::ensure_web_setup,
    external_cli::{cargo, wasm_bindgen, CommandHelpers},
    manifest::package_name,
};

pub use self::args::RunArgs;

mod args;
mod serve;

pub fn run(args: &RunArgs) -> anyhow::Result<()> {
    let cargo_args = args.cargo_args_builder();

    if let Some(RunSubcommands::Web(web_args)) = &args.subcommand {
        ensure_web_setup()?;

        // If targeting the web, run a web server with the WASM build
        println!("Building for WASM...");
        cargo::build::command().args(cargo_args).ensure_status()?;

        println!("Bundling for the web...");
        wasm_bindgen::bundle(&package_name()?, args.profile())?;

        let port = web_args.port;
        let url = format!("http://localhost:{port}");

        // Serving the app is blocking, so we open the page first
        if web_args.do_open {
            if webbrowser::open(&url).is_err() {
                println!("Failed to open the browser automatically, open the app on <{url}>");
            } else {
                println!("Your app is running on <{url}>");
            }
        } else {
            println!("Open your app on <{url}>");
        }

        serve::serve(port, args.profile())?;
    } else {
        // For native builds, wrap `cargo run`
        cargo::run::command().args(cargo_args).ensure_status()?;
    }

    Ok(())
}
