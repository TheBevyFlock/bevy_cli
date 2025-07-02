# Rust Analyzer Support

If your code editor or IDE supports [Rust Analyzer](https://rust-analyzer.github.io), you can configure it to use `bevy_lint` instead of `cargo check`. This will let you easily see Bevy-specific warnings and quick fixes in your project, alongside the normal checks like `dead_code` and `unexpected_cfgs`.

To do this, you will need to override the `rust-analyzer.check.overrideCommand` configuration value:

```json
{
    "rust-analyzer.check.overrideCommand": [
        "bevy_lint",
        "--workspace",
        "--all-targets",
        "--message-format=json-diagnostic-rendered-ansi",
    ]
}
```

Check Rust Analyzer's and your editor's docs to see where to set this configuration. For example, it is [`.vscode/settings.json` for VSCode](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer#configuration) and [`languages.toml` for Helix](https://github.com/helix-editor/helix/wiki/Language-Server-Configurations#rust).

If your editor does not support colorful diagnostics, you may need to set the message format to `--message-format=json` instead.
