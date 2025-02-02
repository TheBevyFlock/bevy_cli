//! Serving the app locally for the browser.
use actix_web::{rt, web, App, HttpResponse, HttpServer, Responder};

use crate::web::bundle::{Index, LinkedBundle, PackedBundle, WebBundle};

/// Serve a static HTML file with the given content.
async fn serve_static_html(content: &'static str) -> impl Responder {
    // Build the HTTP response with appropriate headers to serve the content as a file
    HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/html; charset=utf-8",
        ))
        .body(content)
}

/// Serve a static JavaScript file with the given content.
async fn serve_static_js(content: &'static str) -> impl Responder {
    // Build the HTTP response with appropriate headers to serve the content as a file
    HttpResponse::Ok()
        .insert_header((
            actix_web::http::header::CONTENT_TYPE,
            "text/javascript; charset=utf-8",
        ))
        .body(content)
}

/// Launch a web server running the Bevy app.
pub(crate) fn serve(web_bundle: WebBundle, port: u16) -> anyhow::Result<()> {
    rt::System::new().block_on(
        HttpServer::new(move || {
            let mut app = App::new();

            match web_bundle.clone() {
                WebBundle::Packed(PackedBundle { path }) => {
                    app = app.service(actix_files::Files::new("/", path).index_file("index.html"));
                }
                WebBundle::Linked(LinkedBundle {
                    build_artifact_path,
                    wasm_file_name,
                    js_file_name,
                    index,
                    assets_path,
                }) => {
                    // Serve the build artifacts at the `/build/*` route
                    // A custom `index.html` will have to call `/build/{bin_name}.js`
                    app = app
                        .service(
                            actix_files::Files::new("/build", build_artifact_path)
                                // This potentially includes artifacts which we will not need,
                                // but we can't add the bin name to the check due to lifetime
                                // requirements
                                .path_filter(move |path, _| {
                                    path.file_name() == Some(&js_file_name)
                                        || path.file_name() == Some(&wasm_file_name)
                                }),
                        )
                        // Serve the script to automatically reload the page on changes
                        .route(
                            "/_bevy_dev/auto_reload.js",
                            web::get().to(move || {
                                serve_static_js(include_str!(concat!(
                                    env!("CARGO_MANIFEST_DIR"),
                                    "/assets/web/_bevy_dev/auto_reload.js"
                                )))
                            }),
                        );

                    // If the app has an assets folder, serve it under `/assets`
                    if let Some(assets_path) = assets_path {
                        app = app.service(actix_files::Files::new("/assets", assets_path))
                    }

                    match index {
                        Index::Folder(path) => {
                            app = app.service(
                                actix_files::Files::new("/", path).index_file("index.html"),
                            );
                        }
                        Index::Static(contents) => {
                            app = app.route("/", web::get().to(move || serve_static_html(contents)))
                        }
                    }
                }
            }

            app
        })
        .bind(("127.0.0.1", port))?
        .run(),
    )?;

    Ok(())
}
