//! Checks for adding systems to a disallowed schedule.
//!
//! - `fixed_update_schedule`: Disallows using the `FixedUpdate` schedule.
//! - `update_schedule`: Disallows using the `Update` schedule.
//!
//! # Motivation
//!
//! Often, projects will prefer certain systems be in either the `Update` or `FixedUpdate`
//! schedule. These two lints are useful for denying an unwanted schedule throughout entire
//! modules. For example, a project may deny the `Update` schedule in a module that only contains
//! physics logic, or deny the `FixedUpdate` schedule in a module that likewise contains only
//! rendering code.
//!
//! # Example
//!
//! ```
//! mod physics {
//!     #![warn(bevy::update_schedule)]
//!
//!     fn plugin(app: &mut App) {
//!         // This isn't allowed, use `FixedUpdate` instead!
//!         app.add_systems(Update, my_system);
//!     }
//!
//!     fn my_system() {
//!         // ...
//!     }
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! mod physics {
//!     #![warn(bevy::update_schedule)]
//!
//!     fn plugin(app: &mut App) {
//!         app.add_systems(FixedUpdate, my_system);
//!     }
//!
//!     fn my_system() {
//!         // ...
//!     }
//! }
//! ```

use clippy_utils::diagnostics::span_lint_hir;
use rustc_lint::{LateContext, LateLintPass, Lint};
use rustc_middle::ty::Ty;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, sym, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub(crate) UPDATE_SCHEDULE,
    super::Restriction,
    "defined a system in the `FixedUpdate` schedule",
}

declare_bevy_lint! {
    pub(crate) FIXED_UPDATE_SCHEDULE,
    super::Restriction,
    "defined a system in the `Update` schedule",
}

declare_bevy_lint_pass! {
    pub(crate) Schedule => [UPDATE_SCHEDULE, FIXED_UPDATE_SCHEDULE],
}

impl<'tcx> LateLintPass<'tcx> for Schedule {
    fn check_expr(
        &mut self,
        cx: &rustc_lint::LateContext<'tcx>,
        expr: &'tcx rustc_hir::Expr<'tcx>,
    ) {
        if expr.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }

        let Some(MethodCall {
            method_path,
            args,
            receiver,
            ..
        }) = MethodCall::try_from(cx, expr)
        else {
            return;
        };

        let receiver_ty = cx.typeck_results().expr_ty_adjusted(receiver).peel_refs();

        // Match calls to `App::add_systems(schedule, systems)`
        if !crate::paths::APP.matches_ty(cx, receiver_ty)
            || method_path.ident.name != sym::add_systems
        {
            return;
        }

        // First argument must be the `ScheduleLabel`
        let Some(schedule_label) = args.first() else {
            return;
        };

        let schedule_ty = cx.typeck_results().expr_ty_adjusted(schedule_label);

        let Some(schedule_type) = ScheduleType::try_from_ty(cx, schedule_ty) else {
            return;
        };

        span_lint_hir(
            cx,
            schedule_type.lint(),
            schedule_label.hir_id,
            schedule_label.span,
            format!("the `{}` schedule is disallowed", schedule_type.name()),
        );
    }
}

enum ScheduleType {
    FixedUpdate,
    Update,
}

impl ScheduleType {
    /// Returns the corresponding variant for the given [`Ty`], if it is supported by this lint.
    fn try_from_ty<'tcx>(cx: &LateContext<'tcx>, ty: Ty<'tcx>) -> Option<Self> {
        if crate::paths::FIXED_UPDATE.matches_ty(cx, ty) {
            Some(Self::FixedUpdate)
        } else if crate::paths::UPDATE.matches_ty(cx, ty) {
            Some(Self::Update)
        } else {
            None
        }
    }

    fn name(&self) -> &'static str {
        match self {
            ScheduleType::FixedUpdate => "FixedUpdate",
            ScheduleType::Update => "Update",
        }
    }

    fn lint(&self) -> &'static Lint {
        match self {
            Self::FixedUpdate => FIXED_UPDATE_SCHEDULE,
            Self::Update => UPDATE_SCHEDULE,
        }
    }
}
