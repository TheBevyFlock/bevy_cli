//! Checks for calls to `Events::<T>::iter_current_update_events()`.
//!
//! # Motivation
//!
//! `Events::<T>::iter_current_update_events()` lets you read all of the current events since
//! `Events::<T>::update()` was last called, similar to `EventReader<T>`. Unlike `EventReader<T>`,
//! `iter_current_update_events()` does not track which events have already been read. As such,
//! `iter_current_update_events()` is highly discouraged because it may skip events or yield them
//! multiple times.
//!
//! # Example
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Event)]
//! struct MyEvent;
//!
//! fn my_system(events: Res<Events<MyEvent>>) {
//!     for event in events.iter_current_update_events() {
//!         // ...
//!     }
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! # use bevy::prelude::*;
//! #
//! #[derive(Event)]
//! struct MyEvent;
//!
//! fn my_system(mut events: EventReader<MyEvent>) {
//!     for event in events.read() {
//!         // ...
//!     }
//! }
//! ```

use clippy_utils::{diagnostics::span_lint_and_help, ty::match_type};
use rustc_hir::Expr;
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub ITER_CURRENT_UPDATE_EVENTS,
    SUSPICIOUS,
    "called `Events::<T>::iter_current_update_events()`",
}

declare_bevy_lint_pass! {
    pub IterCurrentUpdateEvents => [ITER_CURRENT_UPDATE_EVENTS.lint],

    @default = {
        iter_current_update_events: Symbol = Symbol::intern("iter_current_update_events"),
    },
}

impl<'tcx> LateLintPass<'tcx> for IterCurrentUpdateEvents {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if let Some(method_call) = MethodCall::try_from(cx, expr) {
            // Find the adjusted type of the receiver. Type adjustment does things like
            // auto-dereference and type coercion. In this example, we use the adjusted type so
            // that we can also handle `Res<Events<T>>`.
            //
            // ```
            // fn plain(events: Events<T>) {
            //     // Original type is `Events<T>`, adjusted type is `Events<T>`.
            //     let _ = events.iter_current_update_events();
            // }
            //
            // fn res(events: Res<Events<T>>) {
            //     // Original type is `Res<Events<T>>`, adjusted type is `Events<T>`.
            //     let _ = events.iter_current_update_events();
            // }
            // ```
            let src_ty = cx
                .typeck_results()
                .expr_ty_adjusted(method_call.receiver)
                .peel_refs();

            if !match_type(cx, src_ty, &crate::paths::EVENTS) {
                return;
            }

            if method_call.method_path.ident.name == self.iter_current_update_events {
                span_lint_and_help(
                    cx,
                    ITER_CURRENT_UPDATE_EVENTS.lint,
                    method_call.span,
                    ITER_CURRENT_UPDATE_EVENTS.lint.desc,
                    None,
                    "`iter_current_update_events()` does not track which events have already been seen, consider using `EventReader<T>` instead",
                );
            }
        }
    }
}
