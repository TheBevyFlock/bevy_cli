//! Serving the app locally for the browser.
use axum::{
    Router,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    middleware::map_response,
    response::Response,
    routing::{any, get},
};
use http::HeaderMap;
use std::net::SocketAddr;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

use crate::web::bundle::{Index, LinkedBundle, PackedBundle, WebBundle};

async fn dev_websocket(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(msg) = msg {
            if socket.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    }
}

/// Launch a web server running the Bevy app.
#[tokio::main]
pub(crate) async fn serve(
    web_bundle: WebBundle,
    port: u16,
    header_map: HeaderMap,
) -> anyhow::Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // PERF: Leaking necessary to satisfy lifetime bounds
    let header_map = Box::leak(Box::new(header_map));

    let mut router = Router::new();

    if !header_map.is_empty() {
        tracing::debug!("Adding response headers {header_map:?}");
    }

    match web_bundle.clone() {
        WebBundle::Packed(PackedBundle { path }) => {
            router = router.route_service("/", ServeDir::new(path));
        }
        WebBundle::Linked(LinkedBundle {
            build_artifact_path,
            wasm_file_name,
            js_file_name,
            index,
            assets_path,
        }) => {
            router = router
                .route_service(
                    &format!("/build/{}", js_file_name.to_str().unwrap()),
                    ServeFile::new(build_artifact_path.join(js_file_name)),
                )
                .route_service(
                    &format!("/build/{}", wasm_file_name.to_str().unwrap()),
                    ServeFile::new(build_artifact_path.join(wasm_file_name)),
                )
                .route(
                    "/_bevy_dev/auto_reload.js",
                    get(async || {
                        (
                            [(http::header::CONTENT_TYPE, "text/javascript; charset=utf-8")],
                            include_str!(concat!(
                                env!("CARGO_MANIFEST_DIR"),
                                "/assets/web/_bevy_dev/auto_reload.js"
                            )),
                        )
                    }),
                )
                // Open a websocket for automatic reloading
                // For now, just echo the messages back
                .route("/_bevy_dev/websocket", any(dev_websocket));

            // If the app has an assets folder, serve it under `/assets`
            if let Some(assets_path) = assets_path {
                router = router.nest_service("/assets", ServeDir::new(assets_path));
            }

            match index {
                Index::File(path) => {
                    router = router.route_service("/", ServeDir::new(path));
                }
                Index::Content(content) => {
                    // Try to inject the auto reload script in the document body
                    // TODO: Do this also for the other cases when the `index.html` is in a
                    // folder
                    let contents = content.replace(
                        "</body>",
                        r#"<script src="_bevy_dev/auto_reload.js"></script></body>"#,
                    );

                    router = router.route(
                        "/",
                        get(async || {
                            (
                                [(http::header::CONTENT_TYPE, "text/html; charset=utf-8")],
                                contents,
                            )
                        }),
                    );
                }
            }
        }
    }

    // Add middlewares
    router = router
        .layer(TraceLayer::new_for_http())
        // Add user-defined headers to every request
        .layer(map_response(async |mut response: Response| {
            let headers = response.headers_mut();
            for (key, value) in header_map.iter() {
                headers.append(key, value.clone());
            }
            response
        }));

    axum::serve(listener, router).await.unwrap();

    Ok(())
}
