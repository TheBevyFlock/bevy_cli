use std::{
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

/// Queries the host tuple from `rustc` and returns it as a string.
pub fn host_tuple() -> String {
    let output = Command::new("rustc")
        .arg("--print=host-tuple")
        // Show errors directly to the user, rather than capturing them.
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run `rustc --print=host-tuple`");

    // `rustc` only works with UTF-8, so it's safe to error if invalid UTF-8 is found.
    str::from_utf8(&output.stdout)
        .expect("`rustc --print=host-tuple` did not emit valid UTF-8")
        // Remove the trailing `\n`.
        .trim_end()
        .to_string()
}

pub trait PathExt {
    /// Converts a UTF-8 Unix path to a native path.
    ///
    /// If run on Windows, this will replace all forward slashes `/` with backslashes `\`. Else,
    /// this will do nothing.
    ///
    /// This will return [`None`] if the Unix path is not valid UTF-8.
    fn unix_to_native(&self) -> Option<PathBuf>;
}

impl PathExt for Path {
    fn unix_to_native(&self) -> Option<PathBuf> {
        if cfg!(windows) {
            self.to_str()
                .map(|path| PathBuf::from(path.replace("/", "\\")))
        } else {
            Some(self.to_path_buf())
        }
    }
}
