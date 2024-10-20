# Creating a Lint

## Finding Lint Ideas

You can find a lint to work on from the issue tracker. Lint ideas are tagged with `A-Linter` and `C-Feature`, so you can [easily filter them](https://github.com/TheBevyFlock/bevy_cli/issues?q=is%3Aopen+is%3Aissue+label%3AA-Linter+label%3AC-Feature). You may also find the [`bevy_lint` Lints project](https://github.com/orgs/TheBevyFlock/projects/3) useful, as it organizes and color-codes lints based on their category and milestone.

If you have your own lint idea, feel free to work on that as well! Consider creating an issue first, though, so other contributors can help you determine the specific details and potential drawbacks. It's easier to work with a clear plan in mind, and we don't want you going to all this work only for the lint to require major changes!

## Lint Naming

Please see [RFC 344](https://rust-lang.github.io/rfcs/0344-conventions-galore.html#lints) for lint naming conventions.

## Setting Up the Lint

Create a new file under the [`lints` module](../src/lints) named `your_lint_name.rs`. Paste the following template:

```rust
//! Short description of what the lint does.
//!
//! # Motivation
//!
//! Why might users want this lint?
//!
//! # Example
//!
//! ```
//! Example that will cause lint to trigger.
//! ```
//!
//! Use instead:
//!
//! ```
//! A fixed version of the above example.
//! ```

use crate::declare_bevy_lint;
use rustc_lint::{EarlyLintPass, LateLintPass};
use rustc_session::declare_lint_pass;

declare_bevy_lint! {
    pub YOUR_LINT_NAME,
    LINT_GROUP,
    "1 sentence description of lint"
}

declare_lint_pass! {
    YourLintName => [YOUR_LINT_PASS.lint],
}

// Pick one:

// If you want to check the AST, implement this!
impl EarlyLintPass for YourLintName {}

// If you want to check the HIR (with type checking results), implement this!
impl<'tcx> LateLintPass<'tcx> for YourLintName {}
```

Next, open up [`lints/mod.rs`](../src/lints/mod.rs), add the module, and register the lint and lint pass:

```diff,rust
// ...

pub mod insert_event_resource;
pub mod main_return_without_appexit;
+ pub mod your_lint_name;

pub(crate) static LINTS: &[&BevyLint] = &[
    insert_event_resource::INSERT_EVENT_RESOURCE,
    main_return_without_appexit::MAIN_RETURN_WITHOUT_APPEXIT,
+   your_lint_name::YOUR_LINT_NAME,
];

// ...

pub(crate) fn register_passes(store: &mut LintStore) {
    store.register_late_pass(|_| Box::new(insert_event_resource::InsertEventResource));
    store.register_late_pass(|_| Box::new(main_return_without_appexit::MainReturnWithoutAppExit));
    // Pick one based on whether you implemented `EarlyLintPass` or `LateLintPass`:
+   store.register_early_pass(|_| Box::new(your_lint_name::YourLintName));
+   store.register_late_pass(|_| Box::new(your_lint_name::YourLintName));
}
```
