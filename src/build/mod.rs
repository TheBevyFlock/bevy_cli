use crate::{
    external_cli::{cargo, wasm_bindgen},
    web,
};

pub(crate) use self::args::BuildArgs;

mod args;

pub(crate) fn build(args: &BuildArgs) -> anyhow::Result<()> {
    if args.is_web {
        web::ensure_setup()?;
    }

    let cargo_args = args.cargo_args();

    if args.is_web {
        println!("Building for WASM...");
        cargo::build().args(cargo_args).status()?;

        println!("Bundling for the web...");
        // FIXME: Properly add package name
        wasm_bindgen::bundle("bevy_app", args.is_release).expect("Failed to bundle for the web");
    } else {
        cargo::build().args(cargo_args).status()?;
    }

    Ok(())
}
