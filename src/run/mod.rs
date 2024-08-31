use crate::{
    external_cli::{cargo, wasm_bindgen},
    mainfest::package_name,
    web,
};

pub(crate) use self::args::RunArgs;

mod args;

pub(crate) fn run(args: &RunArgs) -> anyhow::Result<()> {
    if args.is_web {
        web::ensure_setup()?;
    }

    let cargo_args = args.cargo_args();

    if args.is_web {
        println!("Building for WASM...");
        cargo::build().args(cargo_args).status()?;

        println!("Bundling for the web...");
        wasm_bindgen::bundle(&package_name()?, args.is_release)
            .expect("Failed to bundle for the web");

        let port = 4000;
        println!("Open your app at <http://127.0.0.1:{port}>");
        web::serve(port)?;
    } else {
        cargo::run().args(cargo_args).status()?;
    }

    Ok(())
}
