//! Lints that check over `Cargo.toml` instead of your code.

use super::nursery::duplicate_bevy_dependencies::DUPLICATE_BEVY_DEPENDENCIES;
use crate::declare_bevy_lint_pass;
use cargo_metadata::MetadataCommand;
use clippy_utils::sym;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{config::Input, utils::was_invoked_from_cargo};
use rustc_span::Symbol;

declare_bevy_lint_pass! {
    pub Cargo => [DUPLICATE_BEVY_DEPENDENCIES],
    @default = {
        bevy: Symbol = sym!(bevy),
    },
}

impl LateLintPass<'_> for Cargo {
    fn check_crate(&mut self, cx: &LateContext<'_>) {
        // If rustc was not launched by cargo, skip all cargo based lints
        if !was_invoked_from_cargo() {
            return;
        }

        // Find the path to the file we're compiling. We will spawn the `cargo metadata` command in
        // the same folder as this file so that it can find the correct Cargo project.
        let Input::File(ref path) = cx.tcx.sess.io.input else {
            // A string was passed directly to the compiler, not a file, so we cannot locate the
            // Cargo project.
            return;
        };

        match MetadataCommand::new()
            .current_dir(path.parent().expect("file path must have a parent"))
            .exec()
        {
            Ok(metadata) => {
                super::nursery::duplicate_bevy_dependencies::check(cx, &metadata, self.bevy);
            }
            Err(e) => {
                cx.tcx
                    .dcx()
                    .err(format!("could not read cargo metadata: {e}"));
            }
        }
    }
}
