use clippy_utils::ty::ty_from_hir_ty;
use rustc_lint::{LateContext, LateLintPass};
use rustc_middle::ty::Ty;

use crate::{declare_bevy_lint, declare_bevy_lint_pass};

declare_bevy_lint! {
    pub(crate) BEVY_PLATFORM_ALTERNATIVE_EXISTS,
    super::Style,
    "Used type from the `std` that has an existing alternative from `bevy_platform`",
}

declare_bevy_lint_pass! {
    pub(crate) BevyPlatformAlternativeExists => [BEVY_PLATFORM_ALTERNATIVE_EXISTS],
}

impl<'tcx> LateLintPass<'tcx> for BevyPlatformAlternativeExists {
    fn check_ty(
        &mut self,
        cx: &LateContext<'tcx>,
        hir_ty: &'tcx rustc_hir::Ty<'tcx, rustc_hir::AmbigArg>,
    ) {
        if hir_ty.span.in_external_macro(cx.tcx.sess.source_map()) {
            return;
        }
        let ty = ty_from_hir_ty(cx, hir_ty.as_unambig_ty());

        if let Some(platform_type) = BevyPlatformTypes::try_from_ty(cx, ty) {
            // TODO: lint
        }
        dbg!(ty);
    }
}

enum BevyPlatformTypes {
    Instant,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
    LockResult,
    TryLockError,
    TryLockResult,
    Once,
    OnceState,
    OnceLock,
    Mutex,
    MutexGuard,
    LazyLock,
    Barrier,
    BarrierWaitResult,
    // TODO: atomics
    HashTable,
    HashSet,
    HashMap,
    // cell
    SyncUnsafeCell,
    Exclusive,
}

impl BevyPlatformTypes {
    fn try_from_ty(cx: &LateContext, ty: Ty) -> Option<Self> {
        if crate::paths::bevy_platform_types::STD_INSTANT.matches_ty(cx, ty) {
            dbg!("found");
        }
        None
    }
}
