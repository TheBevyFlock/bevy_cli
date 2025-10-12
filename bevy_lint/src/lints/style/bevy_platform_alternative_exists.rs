use clippy_utils::{diagnostics::span_lint_and_help, source::snippet, ty::ty_from_hir_ty};
use rustc_lint::{LateContext, LateLintPass};

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

        // lower the [`hir::Ty`] to a [`rustc_middle::ty::Ty`]
        let ty = ty_from_hir_ty(cx, hir_ty.as_unambig_ty());

        // Check if for the given `ty` an alternative from `bevy_platform` exists.
        if let Some(bevy_platform_alternative) = BevyPlatformType::try_from_ty(cx, ty) {
            span_lint_and_help(
                cx,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS,
                hir_ty.span,
                BEVY_PLATFORM_ALTERNATIVE_EXISTS.desc,
                None,
                format!(
                    "the type `{}` can be replaced with the `no_std` compatible type
                from `bevy_platform` {}",
                    snippet(cx.tcx.sess, hir_ty.span, ""),
                    bevy_platform_alternative.name(),
                ),
            );
        }
    }
}

macro_rules! declare_bevy_platform_types {
    (
    $(
    $variant:ident => $path:ident
    )
    ,+$(,)?
    ) => {
        #[derive(Copy, Clone)]
        pub enum BevyPlatformType {
            $(
                $variant,
            )+
        }

        impl BevyPlatformType{
            pub const fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant),)+
                }
            }

            pub fn try_from_ty(cx: &LateContext<'_>, ty: rustc_middle::ty::Ty<'_>) -> Option<Self> {
                use crate::paths::bevy_platform_types::*;
                $(
                    if $path.matches_ty(cx, ty) {
                        Some(Self::$variant)
                    } else
                )+
                {
                    None
                }
            }
        }
    };
}

declare_bevy_platform_types! {
    Barrier => BARRIER,
    BarrierWaitResult => BARRIERWAITRESULT,
    HashMap => HASHMAP,
    HashSet => HASHSET,
    Instant => INSTANT,
    LazyLock => LAZYLOCK,
    LockResult => LOCKRESULT,
    Mutex => MUTEX,
    MutexGuard => MUTEXGUARD,
    Once => ONCE,
    OnceLock => ONCELOCK,
    OnceState => ONCESTATE,
    PoisonError => POISONERROR,
    RwLock => RWLOCK,
    RwLockReadGuard => RWLOCKREADGUARD,
    RwLockWriteGuard => RWLOCKWRITEGUARD,
    SyncCell => SYNCCELL,
    SyncUnsafeCell => SYNCUNSAFECELL,
    TryLockError => TRYLOCKERROR,
    TryLockResult => TRYLOCKRESULT,
}
