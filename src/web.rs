//! Utilities for targeting the web.
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

use crate::external_cli::{
    cargo, rustup,
    wasm_bindgen::{self},
};

/// Make sure that the user has installed all tools and set up their repository to target the web.
pub(crate) fn ensure_setup() -> anyhow::Result<()> {
    // `wasm32-unknown-unknown` compilation target
    rustup::install_target_if_needed("wasm32-unknown-unknown", true, false)?;
    // `wasm-bindgen-cli` for bundling
    cargo::install_if_needed(wasm_bindgen::PROGRAM, wasm_bindgen::PACKAGE, true, false)?;

    Ok(())
}

/// If the user didn't provide an `index.html`, serve a default one.
async fn serve_default_index() -> impl Responder {
    let content = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/web/index.html"
    ));

    // Build the HTTP response with appropriate headers to serve the content as a file
    HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/html; charset=utf-8",
        ))
        .body(content)
}

/// Launch a web server running the Bevy app.
pub(crate) fn serve(port: u16, is_release: bool) -> anyhow::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(move || {
            let mut app = App::new();

            // Serve the build artifacts at the `/build/*` route
            // A custom `index.html` will have to call `/build/bevy_app.js`
            let js_path = Path::new("bevy_app.js");
            let wasm_path = Path::new("bevy_app_bg.wasm");
            app = app.service(
                actix_files::Files::new("/build", wasm_bindgen::get_target_folder(is_release))
                    .path_filter(move |path, _| path == js_path || path == wasm_path),
            );

            // If the app has an assets folder, serve it under `/assets`
            if Path::new("assets").exists() {
                app = app.service(actix_files::Files::new("/assets", "./assets"))
            }

            if Path::new("web").exists() {
                // Serve the contents of the `web` folder under `/`, if it exists
                app = app.service(actix_files::Files::new("/", "./web").index_file("index.html"));
            } else {
                // If the user doesn't provide a custom web setup, serve a default `index.html`
                app = app.route("/", web::get().to(serve_default_index))
            }

            app
        })
        .bind(("127.0.0.1", port))?
        .run(),
    )?;

    Ok(())
}
