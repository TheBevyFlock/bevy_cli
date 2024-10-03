//! Serving the app locally for the browser.
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

use crate::external_cli::wasm_bindgen;

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
pub(crate) fn serve(port: u16, profile: &str) -> anyhow::Result<()> {
    let profile = profile.to_string();

    rt::System::new().block_on(
        HttpServer::new(move || {
            let mut app = App::new();

            // Serve the build artifacts at the `/build/*` route
            // A custom `index.html` will have to call `/build/bevy_app.js`
            app = app.service(
                actix_files::Files::new("/build", wasm_bindgen::get_target_folder(&profile))
                    .path_filter(|path, _| wasm_bindgen::is_bindgen_artifact(path)),
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
