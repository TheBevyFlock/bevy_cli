use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{cargo::metadata::Metadata, BinTarget};

pub struct LinkedBundle {
    wasm_path: PathBuf,
    js_path: PathBuf,
    assets_path: Option<PathBuf>,
    custom_index: Option<PathBuf>,
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
        custom_index: None,
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

        // TODO: Copy index

        Ok(WebBundle::Packed(PackedBundle { path: base_path }))
    } else {
        Ok(WebBundle::Linked(linked))
    }
}
