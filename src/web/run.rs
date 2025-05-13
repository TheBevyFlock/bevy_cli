use anyhow::Context as _;
use http::{HeaderMap, HeaderValue};
use tracing::{error, info};

use crate::{
    bin_target::BinTarget,
    build::args::BuildArgs,
    external_cli::cargo::metadata::Metadata,
    run::{
        RunArgs,
        args::{RunSubcommands, RunWebArgs},
    },
};

use super::{build::build_web, serve::serve};

/// Run the app in the browser.
///
/// Requires [`RunSubcommands::Web`] to be defined.
pub(crate) fn run_web(
    args: &RunArgs,
    metadata: &Metadata,
    bin_target: &BinTarget,
) -> anyhow::Result<()> {
    let web_args = match &args.subcommand {
        Some(RunSubcommands::Web(web_args)) => web_args,
        None => &RunWebArgs::default(),
    };

    let header_map = parse_headers(&web_args.headers)?;

    let mut build_args: BuildArgs = args.clone().into();

    // When no target is selected, search for the default-run field and append the binary name
    // as `--bin` flag to only compile the default run target
    if build_args.cargo_args.target_args.bin.is_none()
        && build_args.cargo_args.target_args.example.is_none()
    {
        build_args.cargo_args.target_args.bin = Some(bin_target.bin_name.clone());
    }

    let web_bundle = build_web(&mut build_args, metadata, bin_target)?;

    let port = web_args.port;
    let url = format!("http://localhost:{port}");

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

    serve(web_bundle, port, header_map)?;

    Ok(())
}

fn parse_headers(headers: &[String]) -> anyhow::Result<HeaderMap> {
    let mut header_map = HeaderMap::with_capacity(headers.len());

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
