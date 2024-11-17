# Changelog

All notable user-facing changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## 0.1.0 - 2024-11-DD <!-- TODO: Set the day before this is merged. -->

**All Changes**: [`17834eb...lint-v0.1.0`](https://github.com/TheBevyFlock/bevy_cli/compare/17834eb...lint-v0.1.0)

### Added

- Lint `main_return_without_appexit` to `pedantic` ([#84](https://github.com/TheBevyFlock/bevy_cli/pull/84))
- Lint `insert_event_resource` to `suspicious` ([#86](https://github.com/TheBevyFlock/bevy_cli/pull/86))
- Lint groups `correctness`, `suspicious`, `complexity`, `performance`, `style`, `pedantic`, `restriction`, and `nursery` ([#98](https://github.com/TheBevyFlock/bevy_cli/pull/98))
    - These are based directly on [Clippy's Lint Groups](https://doc.rust-lang.org/stable/clippy/lints.html).
- Lints `panicking_query_methods` and `panicking_world_methods` to `restriction` ([#95](https://github.com/TheBevyFlock/bevy_cli/pull/95))
- Lint `plugin_not_ending_in_plugin` to `style` ([#111](https://github.com/TheBevyFlock/bevy_cli/pull/111))
- Lint `missing_reflect` to `restriction` ([#139](https://github.com/TheBevyFlock/bevy_cli/pull/139))
- Lint `zst_query` to `restriction` ([#168](https://github.com/TheBevyFlock/bevy_cli/pull/168))
