use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{cargo::metadata::Metadata, BinTarget};

pub enum Index {
    File(PathBuf),
    Static(&'static str),
}

pub struct LinkedBundle {
    wasm_path: PathBuf,
    js_path: PathBuf,
    assets_path: Option<PathBuf>,
    index: Index,
}

pub struct PackedBundle {
    path: PathBuf,
}

pub enum WebBundle {
    Linked(LinkedBundle),
    Packed(PackedBundle),
}

pub fn create_bundle(
    metadata: &Metadata,
    profile: &str,
    bin_target: BinTarget,
    packed: bool,
) -> anyhow::Result<WebBundle> {
    let assets_path = Path::new("assets");
    // The "_bg" suffix is needed to reference the bindings created by wasm_bindgen,
    // instead of the artifact created directly by cargo.
    let wasm_file_name = format!("{}_bg.wasm", bin_target.bin_name);
    let js_file_name = format!("{}.js", bin_target.bin_name);

    let linked = LinkedBundle {
        wasm_path: bin_target.artifact_directory.join(&wasm_file_name),
        js_path: bin_target.artifact_directory.join(&js_file_name),
        assets_path: if assets_path.exists() {
            Some(assets_path.to_owned())
        } else {
            None
        },
        // TODO: Determine if index is customized
        index: Index::Static(default_index(&bin_target)),
    };

    if packed {
        let base_path = metadata
            .target_directory
            .join("bevy_web")
            .join(profile)
            .join(bin_target.bin_name);

        // Build artifacts
        fs::create_dir_all(base_path.join("build"))?;
        fs::copy(
            linked.wasm_path,
            base_path.join("build").join(&wasm_file_name),
        )?;
        fs::copy(linked.js_path, base_path.join("build").join(&js_file_name))?;

        // TODO: Copy assets

        let index_path = base_path.join("index.html");
        match linked.index {
            Index::File(path) => {
                fs::copy(path, index_path)?;
            }
            Index::Static(contents) => {
                fs::write(index_path, contents)?;
            }
        }

        Ok(WebBundle::Packed(PackedBundle { path: base_path }))
    } else {
        Ok(WebBundle::Linked(linked))
    }
}

/// Create the default `index.html` if the user didn't provide one.
pub fn default_index(bin_target: &BinTarget) -> &'static str {
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
