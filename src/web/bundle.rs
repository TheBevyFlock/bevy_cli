use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use anyhow::Context;
use cargo_metadata::Metadata;
use tracing::{info, warn};

use crate::bin_target::BinTarget;

#[derive(Debug, Clone)]
pub enum Index {
    /// The path to the custom `index.html` file.
    File(PathBuf),
    /// A static string representing the contents of `index.html`.
    Content(String),
}

impl Index {
    /// The content of the `index.html` file.
    pub fn content(&self) -> anyhow::Result<String> {
        match self {
            Self::File(path) => Ok(fs::read_to_string(path)?),
            Self::Content(content) => Ok(content.clone()),
        }
    }
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
    /// The path to the custom `web` folder, if provided by the user.
    pub web_assets: Option<PathBuf>,
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
    let package_root = bin_target
        .package
        .manifest_path
        .parent()
        .context("failed to find package root")?;

    let assets_path = Path::new("assets");

    let package_assets = Path::new(package_root).join(assets_path);

    let assets_path = if package_assets.exists() {
        info!("using package assets.");
        Some(package_assets)
    } else if assets_path.exists() {
        info!("using workspace assets.");
        Some(assets_path.into())
    } else {
        None
    };

    // The "_bg" suffix is needed to reference the bindings created by wasm_bindgen,
    // instead of the artifact created directly by cargo.
    let wasm_file_name = OsString::from(format!("{}_bg.wasm", bin_target.bin_name));
    let js_file_name = OsString::from(format!("{}.js", bin_target.bin_name));

    let web_assets_folder = Path::new("web");

    let package_web_assets = Path::new(package_root).join(web_assets_folder);
    let workspace_web_assets = Path::new(&metadata.workspace_root).join(web_assets_folder);

    // Search for custom web assets first in the package, then in the workspace
    let web_assets = if package_web_assets.exists() {
        info!("using custom package web assets.");
        Some(package_web_assets)
    } else if workspace_web_assets.exists() {
        info!("using custom workspace web assets.");
        Some(workspace_web_assets)
    } else {
        info!("no custom web assets found, using defaults.");
        None
    };

    let index_path = web_assets
        .as_ref()
        .map(|web_assets| web_assets.join("index.html"));

    let index = if let Some(index_path) = index_path {
        if index_path.exists() {
            Index::File(index_path)
        } else {
            warn!("custom web assets don't contain index.html, using default.");
            Index::Content(default_index(bin_target))
        }
    } else {
        Index::Content(default_index(bin_target))
    };

    let index = pre_process_index(index.content()?, bin_target);

    let linked = LinkedBundle {
        build_artifact_path: bin_target.artifact_directory.clone(),
        wasm_file_name,
        js_file_name,
        assets_path,
        web_assets,
        index: Index::Content(index.clone()),
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
    tracing::debug!(
        "copying build artifacts from file://{}",
        linked.build_artifact_path.to_string_lossy()
    );
    fs::create_dir_all(base_path.join("build"))?;
    fs::copy(
        linked.build_artifact_path.join(&linked.wasm_file_name),
        base_path
            .join("build")
            .join(linked.wasm_file_name.to_string_lossy().as_ref()),
    )
    .context("failed to copy WASM artifact")?;
    fs::copy(
        linked.build_artifact_path.join(&linked.js_file_name),
        base_path
            .join("build")
            .join(linked.js_file_name.to_string_lossy().as_ref()),
    )
    .context("failed to copy JS artifact")?;

    // Snippets (https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html)
    let snippets_path = linked.build_artifact_path.join("snippets");
    if snippets_path.exists() {
        tracing::debug!(
            "copying snippets from file://{}",
            snippets_path.to_string_lossy()
        );
        fs_extra::dir::copy(
            snippets_path,
            base_path.join("build"),
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        )
        .context("failed to copy js snippets")?;
    }

    // Assets
    if let Some(assets_path) = linked.assets_path {
        tracing::debug!(
            "copying assets from file://{}",
            assets_path.to_string_lossy()
        );
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

    // Custom web assets
    if let Some(web_assets) = &linked.web_assets {
        tracing::debug!(
            "copying custom web assets from file://{}",
            web_assets.to_string_lossy()
        );
        fs_extra::dir::copy(
            web_assets,
            &base_path,
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                content_only: true,
                ..Default::default()
            },
        )
        .context("failed to copy custom web assets")?;
    }

    // Index (pre-processed)
    tracing::debug!("writing index.html");
    fs::write(base_path.join("index.html"), &index)
        .context("failed to write processed index.html")?;

    Ok(WebBundle::Packed(PackedBundle {
        path: base_path.into(),
    }))
}

/// Apply pre-processing to the provided `index.html`.
fn pre_process_index(mut content: String, bin_target: &BinTarget) -> String {
    if !content.contains("</title>") {
        content = content.replace(
            "</head>",
            &format!(
                "<title>{}</title></head>",
                default_title(&bin_target.bin_name)
            ),
        );
    }
    content
}

/// Returns the contents of the default `index.html`,
/// customized to use the name of the generated binary.
fn default_index(bin_target: &BinTarget) -> String {
    let template = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/web/index.html"
    ));

    // Insert correct path to JS bindings
    template.replace(
        "./build/bevy_app.js",
        format!("./build/{}.js", bin_target.bin_name).as_str(),
    )
}

/// Generate a title to display on the web page by default.
///
/// The title is based on the binary name, but makes it a bit more human readable.
///
/// bevy_new_2d -> Bevy New 2d
fn default_title(bin_name: &str) -> String {
    bin_name
        .split(['_', '-'])
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Make the first letter of the word uppercase.
///
/// See <https://stackoverflow.com/a/38406885>.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_title() {
        assert_eq!(default_title("bevy_new_2d"), "Bevy New 2d");
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("foo"), "Foo");
    }
}
