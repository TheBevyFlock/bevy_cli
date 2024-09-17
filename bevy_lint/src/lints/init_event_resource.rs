//! TODO

use rustc_lint::LateLintPass;
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_tool_lint! {
    pub bevy::INIT_EVENT_RESOURCE,
    Deny,
    "called `App::init_resource::<Events<T>>() instead of `App::add_event::<T>()`"
}

declare_lint_pass! {
    InitEventResource => [INIT_EVENT_RESOURCE]
}

impl<'tcx> LateLintPass<'tcx> for InitEventResource {}
