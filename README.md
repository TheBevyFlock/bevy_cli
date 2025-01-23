# Bevy CLI

A prototype [Bevy] CLI tool intended to streamline common tasks when working on projects. Please see the [initial scope document] and [original issue] for history and motivation. The CLI's current features include:

- Project generation from a template
- [A custom, Bevy-specific linter](bevy_lint/README.md)
- Out-of-the-box support for building and running your Bevy app in the browser

If you need assistance or want to help, reach out to the [`bevy_cli` working group channel] in the [Bevy Discord].

[Bevy]: https://bevyengine.org
[initial scope document]: https://hackmd.io/cCHAfbtaSviU_MDnbNHKxg
[original issue]: https://github.com/bevyengine/bevy/issues/436
[`bevy_cli` working group channel]: https://discord.com/channels/691052431525675048/1278871953721262090
[Bevy Discord]: https://discord.gg/bevy

## Installation

At this point, the CLI is not published as a package yet and needs to be installed via git:

```cli
cargo install --git https://github.com/TheBevyFlock/bevy_cli --locked bevy_cli
```

## Bevy web apps

The CLI makes it easy to build and run web apps made with Bevy, using `bevy build web` and `bevy run web`.
It takes care of compiling the app to Wasm, creating JavaScript bindings and serving it on a local web server to test it out.
Necessary tools will also be installed automatically.

> [!NOTE]
>
> The arguments you know from `cargo` (like `--release`) must be placed before the `web` subcommand, while the web-specific options (like `--open`) must be placed afterwards, e.g.
>
> ```cli
> bevy run --release web --open
> ```

### Running in the browser

Use the `bevy run web` command to run your app in the browser.
The app will be automatically served on a local web server, use the `--open` flag to automatically open it in the browser.

The server will provide a default `index.html` serving as entrypoint for your app.
It features a loading screen and some other utilities.
If you want to customize it, simply create a `web/index.html` file to override the default behavior.
Other files in the `web` folder will also be included in your application.

### Creating web bundles

To deploy your app on a web server, it's often necessary to bundle the binary, assets and web files into a single folder.
Using `bevy build web --bundle`, the CLI can create this bundle for you automatically.
It will be available in the `target/bevy_web` folder, see the command's output for the full file path.

### Compilation profiles

Web apps have different needs than native builds when it comes to compilation.
For example, binary size is a lot more important for web apps, as it has a big influence on the loading times.

The Bevy CLI provides custom `web` and `web-release` compilation profiles, which are optimized for web apps.
They are used by default for the web sub-commands (depending on the `--release` flag).

The profiles can be customized as usual in `Cargo.toml`:

```toml
[profile.web-release]
inherits = "release"
opt-level = "z"
```

Alternatively, you can change the profile entirely, e.g. `bevy run --profile=foo web`.

### Usage in CI

The CLI may include interactive prompts if parts of the required tooling is not installed on the system.
These prompts will break your pipeline if they are triggered in CI.

To avoid this problem, use the `--yes` flag to automatically confirm the prompts:

```cli
bevy build --yes web
```

## Bevy scaffolding

The `bevy new` command lets you easily scaffold a new Bevy project using a custom template or a minimal template provided by [Bevy](https://github.com/TheBevyFlock/bevy_new_minimal).
Templates are just GitHub repositories and can be specified with the `-t` flag.

### Ussage

If no template is omitted, the default minimal template will be chosen.

```cli
bevy new my-project
```

To use a specific template, provide the full GitHub URL

```cli
bevy new my-project -t https://github.com/TheBevyFlock/bevy_new_2d
```

Additionally, any repo prefixed with `bevy_new_` from the [TheBevyFlock](https://github.com/TheBevyFlock) will be usable via its shortcut form i.e. `2d` will use the template [bevy_new_2d](https://github.com/TheBevyFlock/bevy_new_2d).

## License

The Bevy CLI is licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contributing

Please see [`CONTRIBUTING.md`](CONTRIBUTING.md) for more information!

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
