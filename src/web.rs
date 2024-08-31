//! Utilities for targeting the web.

use actix_web::{rt, App, HttpServer};

use crate::external_cli::{cargo, rustup, wasm_bindgen};

/// Make sure that the user has installed all tools and set up their repository to target the web.
pub(crate) fn ensure_setup() -> anyhow::Result<()> {
    // `wasm32-unknown-unknown` compilation target
    rustup::install_target_if_needed("wasm32-unknown-unknown", true, false)?;
    // `wasm-bindgen-cli` for bundling
    cargo::install_if_needed(wasm_bindgen::PROGRAM, wasm_bindgen::PACKAGE, true, false)?;

    Ok(())
}

/// Launch a web server running the Bevy app.
pub(crate) fn serve(port: u16) -> anyhow::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(|| {
            let mut app = App::new();

            app = app.service(actix_files::Files::new("/", "./web").index_file("index.html"));

            app
        })
        .bind(("127.0.0.1", port))?
        .run(),
    )?;

    Ok(())
}
