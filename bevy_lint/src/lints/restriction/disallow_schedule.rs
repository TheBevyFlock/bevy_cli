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

use clippy_utils::{diagnostics::span_lint_hir_and_then, sym, ty::match_type};
use rustc_errors::Applicability;
use rustc_lint::{LateContext, LateLintPass, Lint};
use rustc_middle::ty::Ty;
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub UPDATE_SCHEDULE,
    super::Restriction,
    "defined a system in the `FixedUpdate` schedule",
}

declare_bevy_lint! {
    pub FIXED_UPDATE_SCHEDULE,
    super::Restriction,
    "defined a system in the `Update` schedule",
}

declare_bevy_lint_pass! {
    pub DenySchedule => [UPDATE_SCHEDULE,FIXED_UPDATE_SCHEDULE],
    @default = {
        add_systems: Symbol = sym!(add_systems),
    },
}

impl<'tcx> LateLintPass<'tcx> for DenySchedule {
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

        let receiver_ty = cx.typeck_results().expr_ty(receiver).peel_refs();

        // Match calls to `App::add_systems(schedule, systems)`
        if !match_type(cx, receiver_ty, &crate::paths::APP)
            || method_path.ident.name != self.add_systems
        {
            return;
        }

        // First argument must be the `ScheduleLabel`
        let Some(schedule_label) = args.first() else {
            return;
        };

        let schedule_ty = cx.typeck_results().expr_ty(schedule_label);

        let Some(schedule_type) = ScheduleType::try_from_ty(cx, schedule_ty) else {
            return;
        };

        span_lint_hir_and_then(
            cx,
            schedule_type.lint(),
            schedule_label.hir_id,
            schedule_label.span,
            format!("the `{}` schedule is disallowed", schedule_type.name()),
            |diag| {
                diag.span_suggestion(
                    schedule_label.span,
                    format!("use the `{}` schedule instead", schedule_type.suggestion()),
                    schedule_type.suggestion(),
                    Applicability::MachineApplicable,
                );
            },
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
        if match_type(cx, ty, &crate::paths::FIXED_UPDATE) {
            Some(Self::FixedUpdate)
        } else if match_type(cx, ty, &crate::paths::UPDATE) {
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

    fn suggestion(&self) -> &'static str {
        match self {
            ScheduleType::FixedUpdate => "Update",
            ScheduleType::Update => "FixedUpdate",
        }
    }
}
