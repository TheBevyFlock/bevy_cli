use std::{fs::remove_dir_all, path::PathBuf};

use fs_extra::dir::{self, CopyOptions};

/// Copy `source_folder` into `target_folder` and replace the previous folder.
///
/// Note that the whole `source_folder` will be copied, not only the contents
pub(crate) fn replace_folder<S, T>(source_folder: S, target_folder: T) -> anyhow::Result<()>
where
    S: Into<String>,
    T: Into<String>,
{
    let source_path = PathBuf::from(source_folder.into());
    let target_dir_path = PathBuf::from(target_folder.into());
    let target_path = target_dir_path.join(source_path.file_name().unwrap());

    if target_path.exists() {
        remove_dir_all(&target_path)?;
    }

    if source_path.exists() {
        dir::copy(source_path, target_dir_path, &CopyOptions::new())?;
    }

    Ok(())
}
