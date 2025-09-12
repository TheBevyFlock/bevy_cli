mod bevy;
mod bevy_lint;

mod prelude {
    use std::path::Path;

    /// Returns the path to the Bevy CLI binary built by Cargo.
    pub fn bevy_exe() -> &'static Path {
        snapbox::cmd::cargo_bin!("bevy")
    }
}
