use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;

use crate::{external_cli::cargo::metadata::Metadata, run::BinTarget};

#[derive(Debug, Clone)]
pub enum Index {
    /// The folder containing a custom `index.html` file.
    Folder(PathBuf),
    /// A static string representing the contents of `index.html`.
    Static(&'static str),
}

#[derive(Debug, Clone)]
pub struct LinkedBundle {
    /// The path to the folder containing the Wasm and JS build artifacts.
    pub build_artifact_path: PathBuf,
    /// The name of the Wasm artifact, in the build folder.
    pub wasm_file_name: OsString,
    /// The name of the JS artifact, in the build folder.
    pub js_file_name: OsString,
    /// The path to the Bevy assets folder, if it exists.
    pub assets_path: Option<PathBuf>,
    /// The index file to serve.
    pub index: Index,
}

#[derive(Debug, Clone)]
pub struct PackedBundle {
    /// The path to the folder containing the packed web bundle.
    pub path: PathBuf,
}

/// A bundle of all the files needed to serve the app in the web.
#[derive(Debug, Clone)]
pub enum WebBundle {
    /// A bundle that needs to be linked together, keeping the files at their original places.
    /// Most useful during development, to avoid additional copy operations and duplication.
    Linked(LinkedBundle),
    /// A bundle packed into a single folder, ready to be deployed on a web server.
    Packed(PackedBundle),
}

/// Create a bundle of all the files needed for serving the app in the web.
///
/// If `packed` is set to `true`, the files will be packed together in a single folder.
/// Use this option e.g. to upload it to a web server.
///
/// Otherwise, the assets and build artifacts will be kept at their original place
/// to avoid duplication.
pub fn create_web_bundle(
    metadata: &Metadata,
    profile: &str,
    bin_target: &BinTarget,
    packed: bool,
) -> anyhow::Result<WebBundle> {
    let assets_path = Path::new("assets");
    // The "_bg" suffix is needed to reference the bindings created by wasm_bindgen,
    // instead of the artifact created directly by cargo.
    let wasm_file_name = OsString::from(format!("{}_bg.wasm", bin_target.bin_name));
    let js_file_name = OsString::from(format!("{}.js", bin_target.bin_name));

    let custom_web_folder = Path::new("web");

    let linked = LinkedBundle {
        build_artifact_path: bin_target.artifact_directory.clone(),
        wasm_file_name,
        js_file_name,
        assets_path: if assets_path.exists() {
            Some(assets_path.to_owned())
        } else {
            None
        },
        index: if custom_web_folder.join("index.html").exists() {
            Index::Folder(custom_web_folder.to_path_buf())
        } else {
            println!("No custom `web` folder found, using defaults.");
            Index::Static(default_index(bin_target))
        },
    };

    if !packed {
        return Ok(WebBundle::Linked(linked));
    }

    let base_path = metadata
        .target_directory
        .join("bevy_web")
        .join(profile)
        .join(&bin_target.bin_name);

    // Remove the previous bundle
    // The error can be ignored, because the folder doesn't need to exist yet
    // and the files will also be overwritten if they already exist
    let _ = fs::remove_dir_all(&base_path);

    // Build artifacts
    fs::create_dir_all(base_path.join("build"))?;
    fs::copy(
        linked.build_artifact_path.join(&linked.wasm_file_name),
        base_path.join("build").join(&linked.wasm_file_name),
    )
    .context("failed to copy WASM artifact")?;
    fs::copy(
        linked.build_artifact_path.join(&linked.js_file_name),
        base_path.join("build").join(&linked.js_file_name),
    )
    .context("failed to copy JS artifact")?;

    // Assets
    if let Some(assets_path) = linked.assets_path {
        fs_extra::dir::copy(
            assets_path,
            &base_path,
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        )
        .context("failed to copy assets")?;
    }

    // Index
    let index_path = base_path.join("index.html");
    match linked.index {
        Index::Folder(path) => {
            fs_extra::dir::copy(
                path,
                &base_path,
                &fs_extra::dir::CopyOptions {
                    overwrite: true,
                    content_only: true,
                    ..Default::default()
                },
            )
            .context("failed to copy custom web assets")?;
        }
        Index::Static(contents) => {
            fs::write(index_path, contents).context("failed to create index.html")?;
        }
    }

    Ok(WebBundle::Packed(PackedBundle { path: base_path }))
}

/// Returns the contents of the default `index.html`,
/// customized to use the name of the generated binary.
fn default_index(bin_target: &BinTarget) -> &'static str {
    let template = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/web/index.html"
    ));

    // Insert correct path to JS bindings
    let index = template.replace(
        "./build/bevy_app.js",
        format!("./build/{}.js", bin_target.bin_name).as_str(),
    );

    // Only static strings can be served in the web app,
    // so we leak the string memory to convert it to a static reference.
    // PERF: This is assumed to be used only once and is needed for the rest of the app running
    // time, making the memory leak acceptable.
    Box::leak(index.into_boxed_str())
}
