# Web Apps

The CLI makes it easy to build and run web apps made with Bevy, using `bevy build web` and `bevy run web`.
It takes care of compiling the app to Wasm, creating JavaScript bindings and serving it on a local web server to test it out.
If you are missing any required tools, the CLI will ask you to install them for you automatically.

> **Note**
>
> The arguments you know from `cargo` (like `--release`) must be placed before the `web` subcommand, while the web-specific options (like `--open`) must be placed afterwards, e.g.
>
> ```sh
> bevy run --release web --open
> ```

## Running in the browser

Use the `bevy run web` command to run your app in the browser.
The app will be automatically served on a local web server, use the `--open` flag to automatically open it in the browser.

The server will provide a default `index.html` serving as entrypoint for your app.
It features a loading screen and some other utilities.
If you want to customize it, simply create a `web/index.html` file to override the default behavior.
Other files in the `web` folder will also be included in your application.
You can view the [default `index.html` here](web/index_template.md).

## Creating web bundles

To deploy your app on a web server, it's often necessary to bundle the binary, assets and web files into a single folder.
Using `bevy build web --bundle`, the CLI can create this bundle for you automatically.
It will be available in the `target/bevy_web` folder, see the command's output for the full file path.

## Compilation profiles

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

## Feature configuration

Often, you want to enable certain features only in development mode or only for native and not web builds.
In your `Cargo.toml` you can enable features or disable default features depending on platform (`native`/`web`) and profile (`dev`/`release`):

```toml
[package.metadata.bevy_cli.native.dev]
features = [
    # Enable asset hot reloading for native dev builds.
    "bevy/file_watcher",
    # Enable embedded asset hot reloading for native dev builds.
    "bevy/embedded_watcher",
]

[package.metadata.bevy_cli.web]
# Disable default features for web builds
default-features = false
```

## Usage in CI

The CLI may include interactive prompts if parts of the required tooling is not installed on the system.
These prompts will break your pipeline if they are triggered in CI.

To avoid this problem, use the `--yes` flag to automatically confirm the prompts:

```sh
bevy build --yes web
```
