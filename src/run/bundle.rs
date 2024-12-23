use std::{
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
};

use super::{cargo::metadata::Metadata, BinTarget};

#[derive(Debug, Clone)]
pub enum Index {
    Folder(PathBuf),
    Static(&'static str),
}

#[derive(Debug, Clone)]
pub struct LinkedBundle {
    pub build_artifact_path: PathBuf,
    pub wasm_file_name: OsString,
    pub js_file_name: OsString,
    pub assets_path: Option<PathBuf>,
    pub index: Index,
}

#[derive(Debug, Clone)]
pub struct PackedBundle {
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub enum WebBundle {
    Linked(LinkedBundle),
    Packed(PackedBundle),
}

pub fn create_web_bundle(
    metadata: &Metadata,
    profile: &str,
    bin_target: BinTarget,
    packed: bool,
) -> anyhow::Result<WebBundle> {
    let assets_path = Path::new("assets");
    // The "_bg" suffix is needed to reference the bindings created by wasm_bindgen,
    // instead of the artifact created directly by cargo.
    let wasm_file_name = OsString::from(format!("{}_bg.wasm", bin_target.bin_name));
    let js_file_name = OsString::from(format!("{}.js", bin_target.bin_name));

    let linked = LinkedBundle {
        build_artifact_path: bin_target.artifact_directory.clone(),
        wasm_file_name,
        js_file_name,
        assets_path: if assets_path.exists() {
            Some(assets_path.to_owned())
        } else {
            None
        },
        // TODO: Determine if index is customized
        index: Index::Static(default_index(&bin_target)),
    };

    if !packed {
        return Ok(WebBundle::Linked(linked));
    }

    let base_path = metadata
        .target_directory
        .join("bevy_web")
        .join(profile)
        .join(bin_target.bin_name);

    // Build artifacts
    fs::create_dir_all(base_path.join("build"))?;
    fs::copy(
        linked.build_artifact_path.join(&linked.wasm_file_name),
        base_path.join("build").join(&linked.wasm_file_name),
    )?;
    fs::copy(
        linked.build_artifact_path.join(&linked.js_file_name),
        base_path.join("build").join(&linked.js_file_name),
    )?;

    // Assets
    if let Some(assets_path) = linked.assets_path {
        fs_extra::dir::copy(
            assets_path,
            base_path.join("assets"),
            &fs_extra::dir::CopyOptions {
                overwrite: true,
                ..Default::default()
            },
        )?;
    }

    // Index
    let index_path = base_path.join("index.html");
    match linked.index {
        Index::Folder(path) => {
            fs::copy(path, index_path)?;
        }
        Index::Static(contents) => {
            fs::write(index_path, contents)?;
        }
    }

    Ok(WebBundle::Packed(PackedBundle { path: base_path }))
}

/// Create the default `index.html` if the user didn't provide one.
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
