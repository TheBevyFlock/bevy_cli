use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use anyhow::Context as _;
use cargo_metadata::Metadata;
use http::{HeaderMap, HeaderValue};
use tracing::{error, info};

use super::{build::build_web, serve::serve};
use crate::commands::{
    build::BuildArgs,
    run::{RunArgs, RunSubcommands, RunWebArgs},
};

/// Run the app in the browser.
///
/// Requires [`RunSubcommands::Web`] to be defined.
pub(crate) fn run_web(args: &mut RunArgs, metadata: &Metadata) -> anyhow::Result<()> {
    let mut build_args: BuildArgs = args.clone().into();

    let web_args = match &mut args.subcommand {
        Some(RunSubcommands::Web(web_args)) => web_args,
        None => &mut RunWebArgs::default(),
    };

    #[cfg(feature = "unstable")]
    if web_args.unstable.web_multi_threading() {
        // Make the document cross-origin isolated,
        // which is required for Wasm multi-threading
        // See also https://developer.mozilla.org/en-US/docs/Web/API/Window/crossOriginIsolated
        web_args.headers.extend([
            "cross-origin-opener-policy=same-origin".to_owned(),
            "cross-origin-embedder-policy=require-corp".to_owned(),
        ]);
    }

    let header_map = parse_headers(web_args.headers.iter())?;

    let web_bundle = build_web(&mut build_args, metadata)?;

    let port = web_args.port;
    let host = IpAddr::from_str(&web_args.host).context("failed to parse host address")?;
    let address = SocketAddr::new(host, port);
    let url = format!("http://{address}");

    // Serving the app is blocking, so we open the page first
    if web_args.open {
        match webbrowser::open(&url) {
            Ok(()) => info!("your app is running at <{url}>!"),
            Err(error) => {
                error!(
                    "failed to open the browser automatically, open the app at <{url}>. (Error: {error:?})"
                );
            }
        }
    } else {
        info!("open your app at <{url}>!");
    }

    serve(web_bundle, address, header_map)?;

    Ok(())
}

fn parse_headers<'a>(headers: impl Iterator<Item = &'a String>) -> anyhow::Result<HeaderMap> {
    let mut header_map = HeaderMap::new();

    for header in headers {
        let (key, value) = header
            .split_once(':')
            .or(header.split_once('='))
            .ok_or_else(|| {
                anyhow::anyhow!("headers must separate name and value with ':' or '='")
            })?;

        header_map.insert(
            // PERF: Leaking is necessary here to satisfy lifetime rules.
            // The memory cost is bounded by the number of headers, which is expected to be low.
            // In any case, the headers are needed until the termination of the program.
            leak_to_static(key),
            HeaderValue::from_str(value).context("invalid header value")?,
        );
    }

    Ok(header_map)
}

/// Create a static reference by leaking the memory.
///
/// # Performance
///
/// Be careful with using this function in order to not exhaust the system's memory.
/// It should only be used when the string is expected to live until the end of the program anyway.
#[cfg(feature = "web")]
pub(crate) fn leak_to_static(s: &str) -> &'static str {
    Box::leak(s.to_owned().into_boxed_str())
}
