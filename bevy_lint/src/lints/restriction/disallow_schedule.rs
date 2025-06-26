//! Checks for systems in disallowed schedules
//!
//! - `disallow_fixed_update`: Disallows using the `FixedUpdate` schedule.
//! - `disallow_update`: Disallows using the `Update` schedule.
//!
//! # Motivation
//!
//! In Bevy, systems can be scheduled in various schedules like `Update` and `FixedUpdate`.
//! However, projects often prefer one of these for most game logic.
//!
//! # Example
//!
//! ```
//! use bevy::prelude::*;
//!
//! fn my_system() {}
//!
//! fn main() {
//!     App::new()
//!         .add_systems(FixedUpdate, my_system);
//! }
//! ```
//!
//! Use instead:
//!
//! ```
//! use bevy::prelude::*;
//!
//! fn my_system() {}
//!
//! fn main() {
//!     App::new()
//!         .add_systems(Update, my_system);
//! }
//! ```

use clippy_utils::{diagnostics::span_lint_hir_and_then, sym, ty::match_type};
use rustc_errors::Applicability;
use rustc_lint::{LateContext, LateLintPass, Lint};
use rustc_middle::ty::Ty;
use rustc_span::Symbol;

use crate::{declare_bevy_lint, declare_bevy_lint_pass, utils::hir_parse::MethodCall};

declare_bevy_lint! {
    pub DISALLOW_FIXED_UPDATE,
    super::Restriction,
    "defined a system in the `FixedUpdate` schedule",
}

declare_bevy_lint! {
    pub DISALLOW_UPDATE,
    super::Restriction,
    "defined a system in the `Update` schedule",
}

declare_bevy_lint_pass! {
    pub DenySchedule => [DISALLOW_FIXED_UPDATE,DISALLOW_UPDATE],
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
            format!(
                "use of the `{}` schedule is disallowed",
                schedule_type.name()
            ),
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
            Self::FixedUpdate => DISALLOW_FIXED_UPDATE,
            Self::Update => DISALLOW_UPDATE,
        }
    }

    fn suggestion(&self) -> &'static str {
        match self {
            ScheduleType::FixedUpdate => "Update",
            ScheduleType::Update => "FixedUpdate",
        }
    }
}
