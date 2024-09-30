use std::{fs::File, io::Read as _};

use toml_edit::{DocumentMut, Item, Value};

/// Get the contents of the manifest file.
fn get_cargo_toml(folder_name: &str) -> anyhow::Result<DocumentMut> {
    let mut file = File::open(format!("{folder_name}/Cargo.toml"))?;

    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content.parse()?)
}

/// Determine the name of the cargo package.
pub(crate) fn package_name() -> anyhow::Result<String> {
    let cargo_toml = get_cargo_toml("./")?;

    if let Item::Value(Value::String(name)) = &cargo_toml["package"]["name"] {
        Ok(name.value().clone())
    } else {
        Err(anyhow::anyhow!("No package name defined in Cargo.toml"))
    }
}
