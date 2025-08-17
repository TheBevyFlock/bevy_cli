use std::{fmt::Display, path::Path};

pub struct FileSize {
    pub bytes: u64,
}

impl FileSize {
    /// Create a new `FileSize` from a byte count.
    pub fn from_bytes(bytes: u64) -> Self {
        Self { bytes }
    }

    /// Try determining the file size of a path.
    /// Handles both files and directories.
    ///
    /// Can be slow for large directories.
    pub fn try_for_path(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        Ok(Self::from_bytes(fs_extra::dir::get_size(&path)?))
    }
}

impl Display for FileSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let units = ["B", "KB", "MB", "GB", "TB"];
        let unit_factor = 1024.0;

        let mut size = self.bytes as f64;

        for unit in units.iter() {
            if size < unit_factor {
                return write!(f, "{:.2} {}", size, unit);
            }
            size /= unit_factor;
        }

        write!(f, "{:.2} {}", size, units.last().unwrap())
    }
}
