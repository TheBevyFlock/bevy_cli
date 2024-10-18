//! Serving the app locally for the browser.
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};
use std::path::Path;

use super::BinTarget;

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
pub(crate) fn serve(bin_target: BinTarget, port: u16) -> anyhow::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(move || {
            let mut app = App::new();

            // Serve the build artifacts at the `/build/*` route
            // A custom `index.html` will have to call `/build/{bin_name}.js`
            app = app.service(
                actix_files::Files::new("/build", bin_target.artifact_directory.clone())
                    // This potentially includes artifacts which we will not need,
                    // but we can't add the bin name to the check due to lifetime requirements
                    .path_filter(|path, _| {
                        path.extension().is_some_and(|ext| ext == "js")
                            || path.ends_with("_bg.wasm")
                    }),
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
                // TODO: Appropriately link to the correct JS bindings
                app = app.route("/", web::get().to(serve_default_index))
            }

            app
        })
        .bind(("127.0.0.1", port))?
        .run(),
    )?;

    Ok(())
}
